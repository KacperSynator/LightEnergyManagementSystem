use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
use std::fmt;
use std::time::Duration;
use tokio::time;
use log::{info, warn, error};
use uuid::{Uuid, uuid};


const PERIPHERAL_NAME_MATCH_FILTER: &str = "LightController";
const NOTIFY_CHARACTERISTIC_UUID: Uuid = uuid!("95b17eef-0276-4e5d-a97b-afc0eff7b4dd");


#[derive(Debug)]
struct BLEConnectionError(String);

impl fmt::Display for BLEConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for BLEConnectionError {}

pub struct BLEConnection {
    manager: Manager,
}

impl BLEConnection {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;

        if adapter_list.is_empty() {
            return Err(Box::new(BLEConnectionError("No Bluetooth adapters found".into())));
        }

        Ok(Self { manager })
    }

    pub async fn scan(&self) -> Result<(), Box<dyn Error>> {
        for adapter in self.manager.adapters().await? {
            info!("Starting scan...");
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Can't scan BLE adapter for connected devices");
            time::sleep(Duration::from_secs(2)).await;
            let peripherals = adapter.peripherals().await?;
    
            if peripherals.is_empty() {
                error!("BLE peripheral devices were not found");
                return Err(Box::new(BLEConnectionError("BLE peripheral devices were not found".into())));
            }

            //  All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                info!(
                    "Peripheral {:?} is connected: {:?}",
                    &local_name, is_connected
                );
                // Check if it's the peripheral we want.
                if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
                    info!("Found matching peripheral {:?}...", &local_name);
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            error!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    let is_connected = peripheral.is_connected().await?;
                    info!(
                        "Now connected ({:?}) to peripheral {:?}.",
                        is_connected, &local_name
                    );
                    if is_connected {
                        info!("Discover peripheral {:?} services...", local_name);
                        peripheral.discover_services().await?;
                        for characteristic in peripheral.characteristics() {
                            info!("Checking characteristic {:?}", characteristic);
                            // Subscribe to notifications from the characteristic with the selected
                            // UUID.
                            if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
                                && characteristic.properties.contains(CharPropFlags::NOTIFY)
                            {
                                info!("Subscribing to characteristic {:?}", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await?;
                                // Print the first 4 notifications received.
                                let mut notification_stream =
                                    peripheral.notifications().await?.take(1);
                                // Process while the BLE connection is not broken or stopped.
                                while let Some(data) = notification_stream.next().await {
                                    info!(
                                        "Received data from {:?} [{:?}]: {:?}",
                                        local_name, data.uuid, data.value
                                    );
                                }
                            }
                        }
                        info!("Disconnecting from peripheral {:?}...", local_name);
                        peripheral.disconnect().await?;
                    }
                } else {
                    info!("Skipping unknown peripheral {:?}", peripheral);
                }
            }
        }
        Ok(())
    }
}
