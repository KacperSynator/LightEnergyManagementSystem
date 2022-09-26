use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Manager, Adapter, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;
use std::fmt;
use std::time::Duration;
use tokio::time;
use log::{debug, info, error};
use uuid::{Uuid, uuid};


const PERIPHERAL_NAME_MATCH_FILTER: &str = "LightController";
const NOTIFY_CHARACTERISTIC_UUID: Uuid = uuid!("95b17eef-0276-4e5d-a97b-afc0eff7b4dd");
const WRITE_CHARACTERISTIC_UUID: Uuid = uuid!("85b17eef-0276-4e5d-a97b-afc0eff7b4dd");


#[derive(Debug)]
struct BLEConnectionError(String);

impl fmt::Display for BLEConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for BLEConnectionError {}

pub struct BLEConnection {
    adapter: Adapter,
}

impl BLEConnection {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await?;
        let mut adapter_list = manager.adapters().await?;

        if adapter_list.is_empty() {
            return Err(Box::new(BLEConnectionError("No Bluetooth adapters found".into())));
        }

        let adapter = adapter_list.remove(0);
        
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices");

        Ok(Self { adapter })
    }

    pub async fn read_devices_data(&self) -> Result<(), Box<dyn Error>> {  
        let peripherals = scan_for_peripherals(&self.adapter).await?;

        for peripheral in peripherals.iter() {
            
            let is_connected = peripheral.is_connected().await?;
            let local_name = get_peripheral_local_name(peripheral).await?;

            debug!(
                "Peripheral {:?} is connected: {:?}",
                &local_name, is_connected
            );

            if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
                info!("Found matching peripheral {:?}...", &local_name);

                if let Err(_) = discover_services(peripheral, &local_name).await {
                    continue;
                }

                subscribe_to_read_characteristic(peripheral).await?;
                    
                info!("Received data: {:?}", read_data(peripheral).await?);
                    
                info!("Disconnecting from read peripheral {:?}...", &local_name);
                peripheral.disconnect().await?;
            
            } else {
                debug!("Skipping unknown peripheral {:?}", peripheral);
            }
        }
        Ok(())
    }

    pub async fn write_to_device(&self, device_mac: &String, msg: &String) -> Result<(), Box<dyn Error> > {
        let peripherals = scan_for_peripherals(&self.adapter).await?;

        for peripheral in peripherals.iter() {
            
            let is_connected = peripheral.is_connected().await?;
            let mac = peripheral.address();

            debug!(
                "Peripheral {:?} is connected: {:?}",
                &mac, is_connected
            );

            if mac.to_string() == *device_mac {
                info!("Found matching write peripheral {:?}...", &mac);
                
                discover_services(peripheral, device_mac).await?;
                send_data(peripheral, &msg).await?;
                    
                info!("Disconnecting from write peripheral {:?}...", mac);
                peripheral.disconnect().await?;
            
            } else {
                debug!("Skipping unknown peripheral {:?}", peripheral);
            }
        }
        Ok(())
    }
}

async fn scan_for_peripherals(adapter: &Adapter) -> Result<Vec<Peripheral>, Box<dyn Error> >  {
    info!("Scanning for peripherals");
    debug!("Adapter info {:?}", adapter.adapter_info().await?);

    time::sleep(Duration::from_millis(100)).await;
    let peripherals = adapter.peripherals().await?;

    if peripherals.is_empty() {
        error!("BLE peripheral devices were not found");
        return Err(Box::new(BLEConnectionError("BLE peripheral devices were not found".into())));
    }

    Ok(peripherals)
}

async fn get_peripheral_local_name(peripheral: &Peripheral) -> Result<String, Box<dyn Error>> {
    let properties = peripheral.properties().await?;
    Ok(properties
        .unwrap()
        .local_name
        .unwrap_or(String::from("(peripheral name unknown)")))
}

async fn read_data(peripheral: &Peripheral) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut notification_stream =
        peripheral.notifications().await?.take(1);
    
    let data = notification_stream.next().await.unwrap();

    info!(
        "Received data from [{:?}]: {:?}",
        data.uuid, data.value
    );

    Ok(data.value)
}

async fn send_data(peripheral: &Peripheral, data: &String) -> Result<(), Box<dyn Error>> {

    for characteristic in peripheral.characteristics() {
        debug!("Checking characteristic {:?}", characteristic);
        
        if characteristic.uuid == WRITE_CHARACTERISTIC_UUID
            && characteristic.properties.contains(CharPropFlags::WRITE)
        {
            info!("Sending data {:?}", characteristic.uuid);
            peripheral.write(&characteristic, data.as_bytes(), WriteType::WithResponse).await?;
        }
    }

    Ok(())
}

async fn discover_services(peripheral: &Peripheral, name: &String) -> Result<(), Box<dyn Error>> {
    if !peripheral.is_connected().await? {
        if let Err(err) = peripheral.connect().await {
            error!("Error connecting to peripheral, skipping: {}", err);
            return Err(Box::new(BLEConnectionError("Error connecting to peripheral, skipping".into())));
        }
    }

    let is_connected = peripheral.is_connected().await?;

    info!(
        "Now connected ({:?}) to peripheral {:?}.",
        is_connected, &name
    );

    if is_connected {
        debug!("Discover peripheral {:?} services...", name);
        peripheral.discover_services().await?;
    }

    Ok(())
}

async fn subscribe_to_read_characteristic(peripheral: &Peripheral) -> Result<(), Box<dyn Error>> {
    for characteristic in peripheral.characteristics() {
        debug!("Checking characteristic {:?}", characteristic);
        
        if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
            && characteristic.properties.contains(CharPropFlags::NOTIFY)
        {
            info!("Subscribing to characteristic {:?}", characteristic.uuid);
            peripheral.subscribe(&characteristic).await?;
        }
    }

    Ok(())
}
