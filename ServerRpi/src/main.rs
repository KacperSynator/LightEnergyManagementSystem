mod db_handler;
mod mqtt_connection;

// use db_handler::DBHandler;
use mqtt_connection::MqttConnection;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const HOST: &str = "tcp://127.0.0.1:1883";
const CLIENT_ID: &str = "ServerRpi";
const KEEP_ALIVE_TIME: u64 = 30;
const WILL_MSG: &str = "ServerRpi disconnected";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    // let db_handler = DBHandler::new()?;
    let mqtt_connection = MqttConnection::new(
        HOST.to_string(),
        CLIENT_ID.to_string(),
        KEEP_ALIVE_TIME,
        WILL_MSG.to_string(),
    )?;
    mqtt_connection
        .publish("test".to_string(), "hello from ServerRpi".to_string())
        .await?;
    mqtt_connection
        .subscribe("test".to_string(), |_client, msg| {
            let msg = msg.unwrap();
            println!(
                "Message arrived with topic: {:?}\n\tPayload: {:?}",
                msg.topic(),
                msg.payload_str()
            );
        })
        .await?;
    // db_handler.get_all_devices()?;

    loop {
        println!("loop");
        sleep(Duration::from_millis(1000));
    }
}
