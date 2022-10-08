use rusqlite::{Connection, Result};
use protobuf::{EnumOrUnknown, Message};
use log::error;
use std::fmt;
use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use lamp_controller::LampData;


#[derive(Debug)]
struct DBHandlerError(String);

impl fmt::Display for DBHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for DBHandlerError {}


pub struct DBHandler {
    connection: Connection,
}

impl DBHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if let Ok(connection) = Connection::open_in_memory() {
            return Ok(Self { connection });
        };

        error!("Can't open database");
        Err(Box::new(DBHandlerError("Can't open database".into())))
    }

    fn create_tables(&self) -> Result<(), Box<dyn Error>> {
        create_devices_table(&self.connection)?;
        create_lamp_data_table(&self.connection)?;

        Ok(())
    }
}

fn create_devices_table(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS devices (
            id_device  INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            mac_address  TEXT NOT NULL
        )",
        (), 
    )?;

    Ok(())
}

fn create_lamp_data_table(connection: &Connection) ->Result<(), Box<dyn Error>> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS LampData (
            id_lamp_data INTEGER PRIMARY KEY,
            id_device INTEGER,
            illuminance REAL,
            voltage REAL,
            current REAL,
            power REAL,
            energy REAL,
            frequency REAL,
            power_factor REAL,
            FOREIGN KEY(id_device) REFERENCES devices(id_device)
        )",
        (), 
    )?;
    
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn connect_to_database() -> Result<(), Box<dyn Error>> {
        DBHandler::new()?;
        Ok(())
    }

    #[test]
    fn create_tables() -> Result<(), Box<dyn Error>> {
        let db_handler = DBHandler::new()?;
        db_handler.create_tables()?;

        Ok(())
    }
}
