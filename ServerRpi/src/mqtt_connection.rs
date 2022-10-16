use log::{debug, info};
use paho_mqtt as mqtt;
use std::error::Error;
use std::time::Duration;

const HOST: &str = "tcp://127.0.0.1:1883";
const CLIENT_ID: &str = "ServerRpi";
const SUBSCRIBE_TOPIC: &str = "test";
const KEEP_ALIVE_TIME: u64 = 30;
const WILL_MSG: &str = "ServerRpi lost connection";

pub struct MqttConnection {
    client: mqtt::AsyncClient,
}

impl MqttConnection {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: mqtt::AsyncClient::new(create_client_options(
                HOST.to_string(),
                CLIENT_ID.to_string(),
            ))?,
        })
    }

    pub async fn publish(&self, topic: String, payload: String) -> Result<(), Box<dyn Error>> {
        debug!("Topic: {}/n/tPayload: {}", topic, payload);
        let msg = mqtt::Message::new(topic, payload, mqtt::QOS_1);
        self.check_connection().await?;
        self.client.publish(msg).await?;

        info!("Published msg");

        Ok(())
    }

    pub async fn subscribe<F>(&self, callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&mqtt::AsyncClient, Option<mqtt::Message>) + 'static,
    {
        self.check_connection().await?;
        self.client.set_message_callback(callback);
        self.client.subscribe(SUBSCRIBE_TOPIC, mqtt::QOS_1).await?;

        info!("Subscribed");

        Ok(())
    }

    async fn connect(&self) -> Result<(), Box<dyn Error>> {
        self.client
            .connect(create_connection_options(
                KEEP_ALIVE_TIME,
                SUBSCRIBE_TOPIC.to_string(),
                WILL_MSG.to_string(),
                false,
            ))
            .await?;

        Ok(())
    }

    async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        self.client.disconnect(None).await?;

        Ok(())
    }

    async fn check_connection(&self) -> Result<(), Box<dyn Error>> {
        debug!("Connected: {}", self.client.is_connected());

        if !self.client.is_connected() {
            self.connect().await?;
        }

        Ok(())
    }
}

fn create_client_options(host: String, id: String) -> mqtt::CreateOptions {
    mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(id)
        .finalize()
}

fn create_connection_options(
    keep_alive_t: u64,
    topic: String,
    will_msg: String,
    clean_session: bool,
) -> mqtt::ConnectOptions {
    let will_msg = mqtt::Message::new(topic, will_msg, mqtt::QOS_1);
    mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(keep_alive_t))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(clean_session)
        .will_message(will_msg)
        .finalize()
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn publish_and_subscribe_success() -> Result<(), Box<dyn Error>> {
        let mqtt_conn = MqttConnection::new()?;
        mqtt_conn
            .publish("test".to_string(), "hello_from_test".to_string())
            .await?;
        mqtt_conn.subscribe(|_, _| {return;}).await?;
        mqtt_conn.disconnect().await?;
        Ok(())
    }
}
