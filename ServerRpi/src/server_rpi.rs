use crate::db_handler;
use crate::mqtt_connection;
use crate::DataPacket;
use crate::Devices;
use crate::MqttCommand;
use crate::MqttPayload;

use db_handler::DBHandler;
use log::{debug, info, warn};
use mqtt_connection::MqttConnection;
use protobuf::{EnumOrUnknown, Message, SpecialFields};
use std::error::Error;
use std::fmt;

const HOST: &str = "localhost";
const PORT: u16 = 1883;
const CLIENT_ID: &str = "ServerRpi";
const KEEP_ALIVE_TIME: u64 = 30;
const WILL_TOPIC: &str = "u/will";
const WILL_MSG: &str = "ServerRpi disconnected";
const MSG_BUF_SIZE: usize = 10;
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
                PORT,
                KEEP_ALIVE_TIME,
                WILL_TOPIC.to_string(),
                WILL_MSG.to_string(),
                MSG_BUF_SIZE,
            )?,
            db_handler: DBHandler::new()?,
        })
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
            msg.topic, msg.payload
        );

        let mqtt_payload = MqttPayload::parse_from_bytes(&msg.payload)?;

        let (_, sender_id) = parse_topic(&msg.topic);

        match mqtt_payload.command.enum_value_or_default() {
            MqttCommand::GetAllDevices => unsafe {
                get_and_send_devices(&sender_id, &self.db_handler, &self.mqtt_conn).await?
            },
            MqttCommand::HandleDataPacket => {
                parse_and_insert_data_packet(&msg.payload, &self.db_handler)?
            }
            _ => warn!("Unknown topic: {}", msg.topic),
        };

        Ok(())
    }
}

fn parse_and_insert_data_packet(
    payload: &[u8],
    db_handler: &DBHandler,
) -> Result<(), Box<dyn Error>> {
    let parsed_msg = DataPacket::parse_from_bytes(payload);

    if parsed_msg.is_err() {
        return Err(Box::new(ServerRpiError(
            "Failed to parse msg payload".into(),
        )));
    }

    db_handler.insert_data_packet(&parsed_msg.unwrap())?;

    debug!(
        "Parsed msg data_packet: {:?}",
        DataPacket::parse_from_bytes(payload).unwrap_or_default()
    );

    Ok(())
}

async unsafe fn get_and_send_devices(
    sender_id: &String,
    db_handler: &DBHandler,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    let devices = Devices {
        devices: db_handler.get_all_devices()?,
        special_fields: SpecialFields::new(),
    };

    let msg = String::from_utf8_unchecked(devices.write_to_bytes()?);
    let payload = MqttPayload {
        command: EnumOrUnknown::new(MqttCommand::GetAllDevices),
        msg,
        special_fields: SpecialFields::new(),
    };
    let topic = create_publish_topic(sender_id);

    mqtt_conn.publish(topic, payload.to_string()).await?;

    Ok(())
}

fn create_publish_topic(id: &String) -> String {
    format!("d/{id}")
}

fn parse_topic(topic: &str) -> (String, String) {
    let vec: Vec<String> = topic.split('/').map(|str| str.to_string()).collect();

    match &vec[..] {
        [direction, sender_id, ..] => (direction.to_string(), sender_id.to_string()),
        _ => unreachable!(),
    }
}
