use crate::db_handler;
use crate::mqtt_connection;
use crate::DataPacket;
use crate::Device;
use crate::DeviceMeasurements;
use crate::Devices;
use crate::MqttCommand;
use crate::MqttPayload;

use db_handler::DBHandler;
use log::{debug, error, info, warn};
use mqtt_connection::MqttConnection;
use protobuf::{EnumOrUnknown, Message, MessageField, SpecialFields};
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

        info!("MqttPayload: {:?}", &mqtt_payload);

        let (_, sender_id) = parse_topic(&msg.topic);

        match mqtt_payload.command.enum_value_or_default() {
            MqttCommand::GetAllDevices => {
                get_and_send_devices(&sender_id, &self.db_handler, &self.mqtt_conn).await?
            }
            MqttCommand::HandleDataPacket => {
                parse_and_insert_data_packet(&mqtt_payload.msg, &self.db_handler)?
            }
            MqttCommand::GetDeviceMeasurements => {
                get_and_send_device_measurements(
                    &sender_id,
                    &mqtt_payload.msg,
                    &self.db_handler,
                    &self.mqtt_conn,
                )
                .await?
            }
            MqttCommand::GetDeviceMeasurementsAfter => {
                get_and_send_device_measurements_after(
                    &sender_id,
                    &mqtt_payload.msg,
                    &self.db_handler,
                    &self.mqtt_conn,
                )
                .await?
            }
            MqttCommand::GetDeviceMeasurementsBefore => {
                get_and_send_device_measurements_before(
                    &sender_id,
                    &mqtt_payload.msg,
                    &self.db_handler,
                    &self.mqtt_conn,
                )
                .await?
            }
            MqttCommand::ChangeDeviceName => {
                change_device_name(
                    &sender_id,
                    &mqtt_payload.msg,
                    &self.db_handler,
                    &self.mqtt_conn,
                )
                .await?
            }
            _ => warn!("Unknown topic: {}", msg.topic),
        };

        Ok(())
    }
}

fn parse_and_insert_data_packet(
    payload: &Vec<Vec<u8>>,
    db_handler: &DBHandler,
) -> Result<(), Box<dyn Error>> {
    if payload.is_empty() {
        error!("payload is empty, should be of length 1");
        return Err(Box::new(ServerRpiError(
            "payload is empty, should be of length 1".into(),
        )));
    }

    let parsed_msg = DataPacket::parse_from_bytes(&payload[0]);

    if parsed_msg.is_err() {
        return Err(Box::new(ServerRpiError(
            "Failed to parse msg payload".into(),
        )));
    }

    db_handler.insert_data_packet(&parsed_msg.unwrap())?;

    debug!(
        "Parsed msg data_packet: {:?}",
        DataPacket::parse_from_bytes(&payload[0]).unwrap_or_default()
    );

    Ok(())
}

async fn get_and_send_devices(
    sender_id: &String,
    db_handler: &DBHandler,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    let devices = Devices {
        devices: db_handler.get_all_devices()?,
        special_fields: SpecialFields::new(),
    };

    let mqtt_payload = MqttPayload {
        command: EnumOrUnknown::new(MqttCommand::GetAllDevices),
        msg: vec![devices.write_to_bytes()?],
        special_fields: SpecialFields::new(),
    };

    send_mqtt_payload(sender_id, &mqtt_payload, mqtt_conn).await?;

    Ok(())
}

async fn change_device_name(
    sender_id: &String,
    payload: &Vec<Vec<u8>>,
    db_handler: &DBHandler,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    if payload.is_empty() {
        error!("payload is empty, should be of length 1");
        return Err(Box::new(ServerRpiError(
            "payload is empty, should be of length 1".into(),
        )));
    }

    let device = Device::parse_from_bytes(&payload[0])?;

    let msg;

    if let Err(e) = db_handler.update_device_name(&device, &device.name) {
        msg = format!("Error: {}", e);
    } else {
        msg = String::from("ok");
    }

    let mqtt_payload = MqttPayload {
        command: EnumOrUnknown::new(MqttCommand::ChangeDeviceName),
        msg: vec![msg.as_bytes().to_vec()],
        special_fields: SpecialFields::new(),
    };

    send_mqtt_payload(sender_id, &mqtt_payload, mqtt_conn).await?;

    Ok(())
}

async fn get_and_send_device_measurements(
    sender_id: &String,
    payload: &Vec<Vec<u8>>,
    db_handler: &DBHandler,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    if payload.is_empty() {
        error!("payload is empty, should be of length 1");
        return Err(Box::new(ServerRpiError(
            "payload is empty, should be of length 1".into(),
        )));
    }

    let device = Device::parse_from_bytes(&payload[0])?;
    let device_measurements = db_handler.get_all_measurements_of_device(&device)?;
    let data_packet = DataPacket {
        device: MessageField::some(device),
        device_measurements,
        special_fields: SpecialFields::new(),
    };

    let mqtt_payload = MqttPayload {
        command: EnumOrUnknown::new(MqttCommand::GetDeviceMeasurements),
        msg: vec![data_packet.write_to_bytes()?],
        special_fields: SpecialFields::new(),
    };

    send_mqtt_payload(sender_id, &mqtt_payload, mqtt_conn).await?;

    Ok(())
}

async fn get_and_send_device_measurements_after(
    sender_id: &String,
    payload: &Vec<Vec<u8>>,
    db_handler: &DBHandler,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    let (device, timestamp) = parse_payload_with_timestamp(payload)?;

    let device_measurements = db_handler.get_measurements_of_device_after(&device, &timestamp)?;

    let mqtt_payload = create_mqtt_payload_containing_data_packet(device, device_measurements)?;

    send_mqtt_payload(sender_id, &mqtt_payload, mqtt_conn).await?;

    Ok(())
}

async fn get_and_send_device_measurements_before(
    sender_id: &String,
    payload: &Vec<Vec<u8>>,
    db_handler: &DBHandler,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    let (device, timestamp) = parse_payload_with_timestamp(payload)?;

    let device_measurements = db_handler.get_measurements_of_device_until(&device, &timestamp)?;

    let mqtt_payload = create_mqtt_payload_containing_data_packet(device, device_measurements)?;

    send_mqtt_payload(sender_id, &mqtt_payload, mqtt_conn).await?;

    Ok(())
}

fn parse_payload_with_timestamp(payload: &Vec<Vec<u8>>) -> Result<(Device, u64), Box<dyn Error>> {
    if payload.len() != 2 {
        error!("payload is empty, should be of length 1");
        return Err(Box::new(ServerRpiError(
            "payload is empty, should be of length 1".into(),
        )));
    }

    let device = Device::parse_from_bytes(&payload[0])?;
    let timestamp = u64::from_le_bytes(payload[1][0..8].try_into().unwrap_or_default());
    if timestamp == 0 {
        error!("timestamp is 0 probably parse from bytes failed");
        return Err(Box::new(ServerRpiError(
            "timestamp is 0 probably parse from bytes failed".into(),
        )));
    }

    Ok((device, timestamp))
}

fn create_mqtt_payload_containing_data_packet(
    device: Device,
    device_measurements: Vec<DeviceMeasurements>,
) -> Result<MqttPayload, Box<dyn Error>> {
    let data_packet = DataPacket {
        device: MessageField::some(device),
        device_measurements,
        special_fields: SpecialFields::new(),
    };

    Ok(MqttPayload {
        command: EnumOrUnknown::new(MqttCommand::GetDeviceMeasurementsAfter),
        msg: vec![data_packet.write_to_bytes()?],
        special_fields: SpecialFields::new(),
    })
}

async fn send_mqtt_payload(
    sender_id: &String,
    mqtt_payload: &MqttPayload,
    mqtt_conn: &MqttConnection,
) -> Result<(), Box<dyn Error>> {
    let payload;

    unsafe {
        payload = String::from_utf8_unchecked(mqtt_payload.write_to_bytes()?);
    }

    let topic = create_publish_topic(sender_id);

    mqtt_conn.publish(topic, payload).await?;

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
