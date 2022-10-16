mod db_handler;
mod mqtt_connection;

// use db_handler::DBHandler;
use mqtt_connection::MqttConnection;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    // let db_handler = DBHandler::new()?;
    let mqtt_connection = MqttConnection::new()?;
    mqtt_connection
        .publish("test".to_string(), "hello from ServerRpi".to_string())
        .await?;
    mqtt_connection
        .subscribe(|client, msg| {
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

    Ok(())
}
