use crate::ble_connection;
use crate::mqtt_connection;
use crate::DataPacket;
use crate::MqttCommand;
use crate::MqttPayload;

use log::{debug, error, info, warn};
use protobuf::{EnumOrUnknown, Message, SpecialFields};
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use ble_connection::BLEConnection;
use mqtt_connection::MqttConnection;

const HOST: &str = "192.168.1.109";
const PORT: u16 = 1883;
const DEVICE_NAME: &str = "LocalRpi";
const KEEP_ALIVE_TIME: u64 = 30;
const WILL_MSG: &str = "LocalRpi disconnected";
const MSG_BUF_SIZE: usize = 10;
const NET_INTERFACE: &str = "eth0";

#[derive(Debug)]
struct LocalRpiError(String);

impl fmt::Display for LocalRpiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for LocalRpiError {}

pub struct LocalRPi {
    ble_conn: BLEConnection,
    mqtt_conn: MqttConnection,
    device_id: String,
}

impl LocalRPi {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let device_id = read_interface_mac_address(NET_INTERFACE)?;
        let client_id = format!("{}:{}", DEVICE_NAME, device_id);
        let will_topic = create_publish_topic(&device_id);
        Ok(Self {
            ble_conn: BLEConnection::new().await?,
            mqtt_conn: MqttConnection::new(
                HOST.to_string(),
                client_id,
                PORT,
                KEEP_ALIVE_TIME,
                will_topic,
                WILL_MSG.to_string(),
                MSG_BUF_SIZE,
            )?,
            device_id,
        })
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn Error>> {
        let topic = format!("d/{}/#", self.device_id);
        self.mqtt_conn.subscribe(topic).await?;

        Ok(())
    }

    pub async fn send_msg_to(&self, dest_mac: &String, msg: &String) -> Result<(), Box<dyn Error>> {
        self.ble_conn.write_to_device(dest_mac, msg).await?;

        Ok(())
    }

    pub async fn get_and_handle_lamp_controllers_data(&self) -> Result<(), Box<dyn Error>> {
        let data_packets = self.ble_conn.read_devices_data().await?;
        debug!("Received data: {:?}", data_packets);

        for data_packet in data_packets.iter() {
            let parsed_data_packet = DataPacket::parse_from_bytes(data_packet);

            if parsed_data_packet.is_err() {
                return Err(Box::<LocalRpiError>::new(LocalRpiError(format!(
                    "Failed to parse data_packet: [as string] {:?}",
                    String::from_utf8_lossy(data_packet)
                ))));
            }

            let mut parsed_data_packet = parsed_data_packet.unwrap();

            info!("Protobuf data: {:?}", &parsed_data_packet);
            update_data_packet_timestamp(&mut parsed_data_packet)?;

            publish_data_packet(&self.device_id, &parsed_data_packet, &self.mqtt_conn).await?;
        }

        Ok(())
    }

    pub async fn read_next_msg(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = self.mqtt_conn.get_msg().await?;

        if msg.is_none() {
            warn!("Message is none");
            return Ok(());
        }

        let msg = msg.unwrap();

        info!(
            "Message arrived with topic: {:?}\n\tPayload: {:?}",
            msg.topic, msg.payload
        );

        Ok(())
    }
}

async fn publish_data_packet(
    device_id: &String,
    data_packet: &DataPacket,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    let mqtt_payload = MqttPayload {
        command: EnumOrUnknown::new(MqttCommand::HandleDataPacket),
        msg: vec![data_packet.write_to_bytes()?],
        special_fields: SpecialFields::new(),
    };

    let payload;

    unsafe {
        payload = String::from_utf8_unchecked(mqtt_payload.write_to_bytes()?);
    }

    mqtt_conn
        .publish(create_publish_topic(device_id), payload)
        .await?;

    Ok(())
}

fn update_data_packet_timestamp(data_packet: &mut DataPacket) -> Result<(), Box<dyn Error>> {
    if data_packet.device_measurements.is_empty() {
        error!("DataPacket's device_measurements is empty");
        return Err(Box::new(LocalRpiError(
            "DataPacket's lamp_data is empty".into(),
        )));
    }

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    debug!("Current timestamp: {}", timestamp);

    for device_msrt in data_packet.device_measurements.iter_mut() {
        device_msrt.timestamp = timestamp;
    }

    debug!("Updated data packet: {:?}", data_packet);
    Ok(())
}

fn create_publish_topic(id: &String) -> String {
    format!("u/{id}")
}

fn read_interface_mac_address(interface: &str) -> Result<String, Box<dyn Error>> {
    let path = format!("/sys/class/net/{}/address", interface);
    let path = Path::new(&path);

    let mut file = fs::File::open(path)?;
    let mut mac = String::new();
    file.read_to_string(&mut mac)?;
    debug!("MAC address: {}", mac);

    Ok(mac)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DeviceMeasurements;
    use protobuf::SpecialFields;

    #[test]
    fn update_timestamp_some_device_measurements() -> Result<(), Box<dyn Error>> {
        let mut data_packet = DataPacket::new();
        data_packet.device_measurements.push(DeviceMeasurements {
            timestamp: 0,
            measurements: Vec::new(),
            special_fields: SpecialFields::new(),
        });

        update_data_packet_timestamp(&mut data_packet)?;
        assert_ne!(
            data_packet.device_measurements.first().unwrap().timestamp,
            0
        );

        Ok(())
    }

    #[test]
    fn update_timestamp_empty_device_measurements() -> Result<(), Box<dyn Error>> {
        let mut data_packet = DataPacket::new();
        if let Err(_) = update_data_packet_timestamp(&mut data_packet) {
            return Ok(());
        };

        assert!(false, "update timestamp didn't fail");
        Ok(())
    }
}
