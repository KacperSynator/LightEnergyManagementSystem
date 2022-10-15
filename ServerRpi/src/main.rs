mod db_handler;

use db_handler::DBHandler;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let db_handler = DBHandler::new()?;
    db_handler.get_all_devices()?;
    
    Ok(())
}
