use rusqlite::{Connection, Result};
use protobuf::{EnumOrUnknown, Message};
use log::{error, debug};
use std::fmt;
use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use lamp_controller::LampData;


const DATABASE_PATH: &str = "/home/pi/lamps_data.db3";


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
        let connection = Connection::open(DATABASE_PATH)?;
        let db_handler = Self { connection };
        db_handler.create_tables()?;

        Ok(db_handler)
    }

    pub fn add_device(&self, lamp_data: &LampData) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "INSERT INTO devices (name, mac_address) VALUES (?1, ?2)",
            (&lamp_data.device_name, &lamp_data.device_mac)
        )?;

        Ok(())
    }

    pub fn get_devices(&self) -> Result<Vec<LampData>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare("SELECT * FROM devices")?;
        let devices_iter = stmt.query_map([], |row| {
            let mut lamp_data = LampData::new();
            lamp_data.device_name = row.get(1)?;
            lamp_data.device_mac = row.get(2)?;

            Ok(lamp_data)
        })?;

        let devices = devices_iter
                .map(|device| { device.unwrap() })
                .collect::<Vec<_>>();

        debug!("Devices received from db\n\t{:?}", devices);

        Ok( devices )
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

    #[test]
    fn add_default_device() -> Result<(), Box<dyn Error>> {
        let db_handler = DBHandler::new()?;
        let lamp_data = LampData::new();

        db_handler.add_device(&lamp_data)?;

        Ok(())
    }

    #[test]
    fn get_devices() -> Result<(), Box<dyn Error>> {
        let db_handler = DBHandler::new()?;

        db_handler.get_devices()?;

        Ok(())
    }
}
