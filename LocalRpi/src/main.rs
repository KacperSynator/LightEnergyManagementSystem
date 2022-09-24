use std::error::Error;
// use log::{info, warn, error};
mod ble_connection;

use ble_connection::BLEConnection;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let ble_conn = BLEConnection::new().await?;
    ble_conn.scan().await?;

    Ok(())
}