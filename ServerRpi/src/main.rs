mod db_handler;

use db_handler::DBHandler;
use log::info;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let db_handler = DBHandler::new()?;
    
    Ok(())
}
