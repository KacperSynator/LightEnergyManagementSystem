use log::{error, info};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use ServerRpi::server_rpi::ServerRpi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let server_rpi = ServerRpi::new();

    if let Err(e) = server_rpi {
        error!("Failed to initialise ServerRpi: {:?}", e);
        return Ok(());
    }

    let mut server_rpi: ServerRpi = server_rpi.unwrap();
    info!("ServerRpi initialised");

    if let Err(e) = server_rpi.subscribe().await {
        error!("ServerRpi subscribe failed: {:?}", e);
    }

    loop {
        if let Err(e) = server_rpi.read_next_msg().await {
            error!("ServerRpi failed to read next message: {:?}", e);
        }

        sleep(Duration::from_millis(100));
    }
}
