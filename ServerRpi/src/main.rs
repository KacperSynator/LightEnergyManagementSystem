mod db_handler;
mod mqtt_connection;

use db_handler::DBHandler;
use mqtt_connection::MqttConnection;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let db_handler = DBHandler::new()?;
    let mqtt_connection = MqttConnection::new()?;
    mqtt_connection.publish("test".to_string(), "hello from ServerRpi".to_string()).await?;
    db_handler.get_all_devices()?;
    
    Ok(())
}
