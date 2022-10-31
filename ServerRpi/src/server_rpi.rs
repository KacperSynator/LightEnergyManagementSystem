use crate::db_handler;
use crate::mqtt_connection;
use crate::DataPacket;

use db_handler::DBHandler;
use log::{debug, info, warn};
use mqtt_connection::MqttConnection;
use protobuf::Message;
use std::error::Error;
use std::fmt;

const HOST: &str = "tcp://127.0.0.1:1883";
const CLIENT_ID: &str = "ServerRpi";
const KEEP_ALIVE_TIME: u64 = 30;
const WILL_MSG: &str = "ServerRpi disconnected";
const MSG_BUF_SIZE: usize = 25;
const PUB_TOPIC: &str = "d/data_packet";
const SUB_TOPIC: &str = "u/#";

#[derive(Debug)]
struct ServerRpiError(String);

impl fmt::Display for ServerRpiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for ServerRpiError {}

pub struct ServerRpi {
    mqtt_conn: MqttConnection,
    db_handler: DBHandler,
}

impl ServerRpi {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            mqtt_conn: MqttConnection::new(
                HOST.to_string(),
                CLIENT_ID.to_string(),
                KEEP_ALIVE_TIME,
                WILL_MSG.to_string(),
                MSG_BUF_SIZE,
            )?,
            db_handler: DBHandler::new()?,
        })
    }

    pub async fn send_msg(&self, msg: String) -> Result<(), Box<dyn Error>> {
        self.mqtt_conn.publish(PUB_TOPIC.to_string(), msg).await?;

        Ok(())
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn Error>> {
        self.mqtt_conn.subscribe(SUB_TOPIC.to_string()).await?;

        Ok(())
    }

    pub async fn read_next_msg(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = self.mqtt_conn.get_msg().await?;

        if msg.is_none() {
            warn!("Message is none");
            return Ok(());
        }

        let msg = msg.unwrap();

        info!(
            "Message arrived with topic: {:?}\n\tPayload: {:?}",
            msg.topic(),
            msg.payload_str()
        );

        let parsed_msg = DataPacket::parse_from_bytes(msg.payload());

        if parsed_msg.is_err() {
            return Err(Box::new(ServerRpiError(
                "Failed to parse msg payload".into(),
            )));
        }

        self.db_handler.insert_data_packet(&parsed_msg.unwrap())?;

        debug!(
            "Parsed msg data_packet: {:?}",
            DataPacket::parse_from_bytes(msg.payload()).unwrap_or_default()
        );
        Ok(())
    }
}
