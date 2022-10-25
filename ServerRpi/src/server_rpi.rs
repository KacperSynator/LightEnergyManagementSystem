use crate::db_handler;
use crate::mqtt_connection;

use mqtt_connection::MqttConnection;
use db_handler::DBHandler;
use std::error::Error;
use log::info;

const HOST: &str = "tcp://127.0.0.1:1883";
const CLIENT_ID: &str = "ServerRpi";
const KEEP_ALIVE_TIME: u64 = 30;
const WILL_MSG: &str = "ServerRpi disconnected";
const PUB_TOPIC: &str = "test";
const SUB_TOPIC: &str = "test";

pub struct ServerRpi {
    mqtt_conn: MqttConnection,
    db_handler: DBHandler
}

impl ServerRpi {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            mqtt_conn: MqttConnection::new(
                HOST.to_string(),
                CLIENT_ID.to_string(),
                KEEP_ALIVE_TIME,
                WILL_MSG.to_string(),
            )?,
            db_handler: DBHandler::new()?
        })
    }

    pub async fn send_msg(&self, msg: String) -> Result<(), Box<dyn Error>> {
        self.mqtt_conn
            .publish(PUB_TOPIC.to_string(), msg)
            .await?;
        
        Ok(())
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn Error>> {
        self.mqtt_conn
        .subscribe(SUB_TOPIC.to_string(), |_client, msg| {
            let msg = msg.unwrap();
            info!(
                "Message arrived with topic: {:?}\n\tPayload: {:?}",
                msg.topic(),
                msg.payload_str()
            );
        })
        .await?;

        Ok(())
    }
}
