use std::error::Error;
use log::info;
mod ble_connection;

use ble_connection::BLEConnection;
use protobuf::Message;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use light_energy_menagment_system::DataPacket;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let ble_conn = BLEConnection::new().await?;

    ble_conn.write_to_device(&String::from("EC:62:60:93:A4:B2"), &String::from("hello from LocalRPi")).await?;
    let data_packets = ble_conn.read_devices_data().await?;
    info!("Received data: {:?}", data_packets);
    for data_packet in data_packets.iter() {
        info!("Protobuf data: {:?}", DataPacket::parse_from_bytes(data_packet).unwrap());
    }
    
    Ok(())
}
