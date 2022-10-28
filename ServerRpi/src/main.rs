use log::{info, error};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use ServerRpi::server_rpi::ServerRpi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let server_rpi = ServerRpi::new()?;
    info!("ServerRpi initialised");
    
    if let Err(e) = server_rpi.subscribe().await {
        error!("ServerRpi subscribe failed: {:?}", e);
    }

    loop {
        if let Err(e) = server_rpi.send_msg("test".to_string()).await {
            error!("ServerRpi failed to send msg: {:?}", e);
        }
        
        sleep(Duration::from_millis(1000));
    }
}
