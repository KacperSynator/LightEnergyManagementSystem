use crate::ble_connection;
use crate::mqtt_connection;

use protobuf::Message;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use log::{info, debug, error};
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use ble_connection::BLEConnection;
use mqtt_connection::MqttConnection;
use light_energy_menagment_system::DataPacket;

const HOST: &str = "tcp://192.168.1.109:1883";
const CLIENT_ID: &str = "LocalRpi";
const KEEP_ALIVE_TIME: u64 = 30;
const WILL_MSG: &str = "LocalRpi disconnected";
const PUB_TOPIC: &str = "u/data_packet";
const SUB_TOPIC: &str = "d/#";


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
}

impl LocalRPi {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            ble_conn: BLEConnection::new().await?,
            mqtt_conn: MqttConnection::new(
                HOST.to_string(),
                CLIENT_ID.to_string(),
                KEEP_ALIVE_TIME,
                WILL_MSG.to_string(),
            )?,
        })
    }

    pub async fn send_msg_to(&self, dest_mac: &String, msg: &String) -> Result<(), Box<dyn Error>> {
        self.ble_conn.write_to_device(dest_mac, msg).await?;

        Ok(())
    }

    pub async fn get_lamp_controllers_data(&self) -> Result<(), Box<dyn Error>> {
        let data_packets = self.ble_conn.read_devices_data().await?;
        debug!("Received data: {:?}", data_packets);

        for data_packet in data_packets.iter() {
            let mut data_packet = DataPacket::parse_from_bytes(data_packet).unwrap();
            info!("Protobuf data: {:?}", &data_packet);
            update_data_packet_timestamp(&mut data_packet)?;
        }

        Ok(())
    }
}

fn update_data_packet_timestamp(data_packet: &mut DataPacket) -> Result<(), Box<dyn Error>> {
    if data_packet.lamp_data.is_none() {
        error!("DataPacket's lamp_data is empty");
        return Err(Box::new(LocalRpiError("DataPacket's lamp_data is empty".into())));
    }

    let mut lamp_data = data_packet.lamp_data.as_mut().unwrap();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    debug!("Current timestamp: {}", timestamp);
    lamp_data.timestamp = timestamp as u32;
    debug!("Updated data packet: {:?}", data_packet);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use light_energy_menagment_system::LampData;
    use protobuf::MessageField;

    #[test]
    fn update_timestamp_some_lamp_data() -> Result<(), Box<dyn Error>> {
        let mut data_packet = DataPacket::new();
        data_packet.lamp_data = MessageField::some(LampData::new());
        update_data_packet_timestamp(&mut data_packet)?;
        assert_ne!(data_packet.lamp_data.into_option().unwrap().timestamp, 0);
        Ok(())
    }

    #[test]
    fn update_timestamp_empty_lamp_data() -> Result<(), Box<dyn Error>> {
        let mut data_packet = DataPacket::new();
        if let Err(_) = update_data_packet_timestamp(&mut data_packet) {
            return Ok(());
        };

        assert!(false, "update timestamp didn't fail");
        Ok(())
    }
}
