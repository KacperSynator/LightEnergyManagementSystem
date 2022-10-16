use paho_mqtt as mqtt;
use std::error::Error;


const BROKER_ADDRESS: &str = "tcp://127.0.0.1:1883";

pub struct MqttConnection {
    client: mqtt::AsyncClient,
}


impl MqttConnection {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok( Self { client: mqtt::AsyncClient::new(BROKER_ADDRESS)? })
    }

    pub async fn publish(&self, topic: String, payload: String) -> Result<(), Box<dyn Error>> {
        let msg = mqtt::Message::new(topic, payload, mqtt::QOS_1);
        self.client.connect(None).await?;
        self.client.publish(msg).await?;
        self.client.disconnect(None).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn publish_test() -> Result<(), Box<dyn Error>> {
        let mqtt_conn = MqttConnection::new()?;
        mqtt_conn.publish("test".to_string(), "hello".to_string()).await?;
        Ok(())
    }
}
