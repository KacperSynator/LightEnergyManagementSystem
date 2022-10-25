use log::{debug, info};
use paho_mqtt as mqtt;
use std::error::Error;
use std::time::Duration;

pub struct MqttConnection {
    client: mqtt::AsyncClient,
    keep_alive_time: u64,
    will_msg: String,
}

impl MqttConnection {
    pub fn new(
        host: String,
        client_id: String,
        keep_alive_time: u64,
        will_msg: String,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: mqtt::AsyncClient::new(create_client_options(host, client_id))?,
            keep_alive_time,
            will_msg,
        })
    }

    pub async fn publish(&self, topic: String, payload: String) -> Result<(), Box<dyn Error>> {
        debug!("Topic: {}/n/tPayload: {}", topic, payload);
        let msg = mqtt::Message::new(topic.clone(), payload, mqtt::QOS_1);
        self.check_connection(topic).await?;
        self.client.publish(msg).await?;

        info!("Published msg");

        Ok(())
    }

    pub async fn subscribe<F>(&self, topic: String, callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&mqtt::AsyncClient, Option<mqtt::Message>) + 'static,
    {
        self.check_connection(topic.clone()).await?;
        self.client.set_message_callback(callback);
        self.client.subscribe(topic, mqtt::QOS_1).await?;

        info!("Subscribed");

        Ok(())
    }

    async fn connect(&self, topic: String) -> Result<(), Box<dyn Error>> {
        connect(
            self.client.clone(),
            topic,
            self.keep_alive_time,
            self.will_msg.clone(),
        )
        .await?;

        Ok(())
    }

    async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        self.client.disconnect(None).await?;

        Ok(())
    }

    async fn check_connection(&self, topic: String) -> Result<(), Box<dyn Error>> {
        debug!("Connected: {}", self.client.is_connected());

        if !self.client.is_connected() {
            self.connect(topic).await?;
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

async fn connect(
    client: mqtt::AsyncClient,
    topic: String,
    keep_alive_time: u64,
    will_msg: String,
) -> Result<(), Box<dyn Error>> {
    client
        .connect(create_connection_options(
            keep_alive_time,
            topic,
            will_msg,
            false,
        ))
        .await?;

    Ok(())
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

    const TEST_HOST: &str = "tcp://127.0.0.1:1883";
    const TEST_CLIENT_ID: &str = "TestServerRpi";
    const TEST_KEEP_ALIVE_TIME: u64 = 5;
    const TEST_WILL_MSG: &str = "Test ServerRpi disconnected";
    const TEST_PUB_TOPIC: &str = "test";
    const TEST_PUB_PAYLOAD: &str = "hello_from_test";
    const TEST_SUB_TOPIC: &str = "test";

    #[tokio::test]
    async fn publish_and_subscribe_success() -> Result<(), Box<dyn Error>> {
        let mqtt_conn = MqttConnection::new(
            TEST_HOST.to_string(),
            TEST_CLIENT_ID.to_string(),
            TEST_KEEP_ALIVE_TIME,
            TEST_WILL_MSG.to_string(),
        )?;
        mqtt_conn
            .publish(TEST_PUB_TOPIC.to_string(), TEST_PUB_PAYLOAD.to_string())
            .await?;
        mqtt_conn
            .subscribe(TEST_SUB_TOPIC.to_string(), |_, msg| {
                if let Some(msg) = msg {
                    assert_eq!(msg.topic(), TEST_PUB_TOPIC);
                    assert_eq!(msg.payload_str(), TEST_PUB_PAYLOAD);
                };
            })
            .await?;
        mqtt_conn.disconnect().await?;
        Ok(())
    }
}
