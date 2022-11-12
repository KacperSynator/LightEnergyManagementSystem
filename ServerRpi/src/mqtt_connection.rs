use log::{debug, info};
use rumqttc::{AsyncClient, Event, EventLoop, LastWill, MqttOptions, Packet, Publish, QoS};
use std::error::Error;
use std::time::Duration;

pub struct MqttConnection {
    client: AsyncClient,
    event_loop: EventLoop,
}

impl MqttConnection {
    pub fn new(
        host: String,
        client_id: String,
        port: u16,
        keep_alive_time: u64,
        will_topic: String,
        will_msg: String,
        msg_buf_size: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let (client, event_loop) = create_client_and_event_loop(
            host,
            client_id,
            port,
            keep_alive_time,
            msg_buf_size,
            will_topic,
            will_msg,
        );
        Ok(Self { client, event_loop })
    }

    pub async fn publish(&self, topic: String, payload: String) -> Result<(), Box<dyn Error>> {
        debug!("Topic: {}/n/tPayload: {}", topic, payload);
        self.client
            .publish(topic, QoS::ExactlyOnce, false, payload)
            .await?;

        info!("Published msg");

        Ok(())
    }

    pub async fn subscribe(&self, topic: String) -> Result<(), Box<dyn Error>> {
        self.client.subscribe(topic, QoS::ExactlyOnce).await?;

        info!("Subscribed");

        Ok(())
    }

    pub async fn get_msg(&mut self) -> Result<Option<Publish>, Box<dyn Error>> {
        let notification = self.event_loop.poll().await?;
        debug!("notification: {:?}", notification);
        if let Event::Incoming(Packet::Publish(msg)) = notification {
            return Ok(Some(msg));
        }

        Ok(None)
    }
}

fn create_client_and_event_loop(
    host: String,
    id: String,
    port: u16,
    keep_alive_time: u64,
    msg_buf_size: usize,
    will_topic: String,
    will_msg: String,
) -> (AsyncClient, EventLoop) {
    let mut options = MqttOptions::new(id, host, port);
    options.set_keep_alive(Duration::from_secs(keep_alive_time));
    options.set_last_will(LastWill::new(will_topic, will_msg, QoS::ExactlyOnce, false));

    AsyncClient::new(options, msg_buf_size)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_HOST: &str = "127.0.0.1";
    const PORT: u16 = 1883;
    const TEST_CLIENT_ID: &str = "TestServerRpi";
    const TEST_KEEP_ALIVE_TIME: u64 = 5;
    const TEST_WILL_TOPIC: &str = "test";
    const TEST_WILL_MSG: &str = "Test ServerRpi disconnected";
    const TEST_PUB_TOPIC: &str = "test";
    const TEST_PUB_PAYLOAD: &str = "hello_from_test";
    const TEST_SUB_TOPIC: &str = "test";
    const TEST_MSG_BUF_SIZE: usize = 10;

    #[tokio::test]
    async fn publish_and_subscribe_success() -> Result<(), Box<dyn Error>> {
        let mut mqtt_conn = MqttConnection::new(
            TEST_HOST.to_string(),
            TEST_CLIENT_ID.to_string(),
            PORT,
            TEST_KEEP_ALIVE_TIME,
            TEST_WILL_TOPIC.to_string(),
            TEST_WILL_MSG.to_string(),
            TEST_MSG_BUF_SIZE,
        )?;
        mqtt_conn.subscribe(TEST_SUB_TOPIC.to_string()).await?;
        mqtt_conn
            .publish(TEST_PUB_TOPIC.to_string(), TEST_PUB_PAYLOAD.to_string())
            .await?;
        let msg = mqtt_conn.get_msg().await?;
        if let Some(msg) = msg {
            assert_eq!(msg.topic, TEST_PUB_TOPIC);
            assert_eq!(&msg.payload, TEST_PUB_PAYLOAD);
        };

        Ok(())
    }
}
