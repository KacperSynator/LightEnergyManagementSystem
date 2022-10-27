use std::error::Error;
use log::info;
use std::thread::sleep;
use std::time::Duration;

use LocalRpi::local_rpi::LocalRPi;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let local_rpi = LocalRPi::new().await?;

    // local_rpi.send_msg_to(&String::from("EC:62:60:93:9D:80"), &String::from("hello from LocalRPi")).await?;
    
    loop {
        info!("Reading data");
        local_rpi.get_and_handle_lamp_controllers_data().await?;
        sleep(Duration::from_millis(1000));
    }
}
