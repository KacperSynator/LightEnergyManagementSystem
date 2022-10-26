use crate::ble_connection;

use protobuf::Message;
use std::error::Error;
use log::{info, debug};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use ble_connection::BLEConnection;
use light_energy_menagment_system::DataPacket;

pub struct LocalRPi {
    ble_conn: BLEConnection,
}

impl LocalRPi {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self{ble_conn: BLEConnection::new().await?})
    }

    pub async fn send_msg_to(&self, dest_mac: &String, msg: &String) -> Result<(), Box<dyn Error>> {
        self.ble_conn.write_to_device(dest_mac, msg).await?;

        Ok(())
    }

    pub async fn get_lamp_controllers_data(&self) -> Result<(), Box<dyn Error>> {
        let data_packets = self.ble_conn.read_devices_data().await?;
        debug!("Received data: {:?}", data_packets);
        for data_packet in data_packets.iter() {
            info!("Protobuf data: {:?}", DataPacket::parse_from_bytes(data_packet).unwrap());
        }

        Ok(())
    }
}
