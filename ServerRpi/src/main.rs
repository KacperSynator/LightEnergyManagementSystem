use ServerRpi::server_rpi::ServerRpi;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let server_rpi = ServerRpi::new()?;
    server_rpi.subscribe().await?;
   

    loop {
        info!("Send msg");
        server_rpi.send_msg("test".to_string()).await?;
        sleep(Duration::from_millis(1000));
    }
}
