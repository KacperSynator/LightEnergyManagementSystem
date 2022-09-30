use std::error::Error;
use log::info;
mod ble_connection;

use ble_connection::BLEConnection;
use protobuf::{EnumOrUnknown, Message};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use lamp_controller::LampData;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let ble_conn = BLEConnection::new().await?;

    ble_conn.write_to_device(&String::from("EC:62:60:93:A4:B2"), &String::from("hello from LocalRPi")).await?;
    let lamps_data = ble_conn.read_devices_data().await?;
    info!("Received data: {:?}", lamps_data);
    for lamp_data in lamps_data.iter() {
        info!("Protobuf data: {:?}", LampData::parse_from_bytes(lamp_data).unwrap());
    }
    
    Ok(())
}
