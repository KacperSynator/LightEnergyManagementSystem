use log::error;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use LocalRpi::local_rpi::LocalRPi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut local_rpi = LocalRPi::new().await?;

    if let Err(e) = local_rpi.subscribe().await {
        error!("LocalRpi failed to subscribe: {:?}", e);
    }

    // if let Err(e) = local_rpi
    //     .send_msg_to(
    //         &String::from("EC:62:60:93:9D:80"),
    //         &String::from("hello from LocalRPi"),
    //     )
    //     .await
    // {
    //     error!("LocalRpi failed to send msg: {:?}", e);
    // }

    loop {
        if let Err(e) = local_rpi.get_and_handle_lamp_controllers_data().await {
            error!("LocalRpi failed to read/handle data: {:?}", e);
        }
        
        for _ in 0..10 {
            if let Err(e) = local_rpi.read_next_msg().await {
                error!("ServerRpi failed to read next message: {:?}", e);
            }
        }
        
        sleep(Duration::from_millis(1000));
    }
}
