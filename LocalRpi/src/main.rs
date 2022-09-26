use std::error::Error;
// use log::{info, warn, error};
mod ble_connection;

use ble_connection::BLEConnection;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let ble_conn = BLEConnection::new().await?;
    ble_conn.write_to_device(&String::from("EC:62:60:93:A4:B2"), &String::from("hello from LocalRPi")).await?;
    ble_conn.read_devices_data().await?;
    
    Ok(())
}