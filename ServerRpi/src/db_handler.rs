use rusqlite::{Connection, Result, params};
use protobuf::{EnumOrUnknown, Message};
use log::{error, debug};
use std::fmt;
use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use lamp_controller::LampData;


const DATABASE_PATH: &str = "./lamps_data.db3";


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
        add_device_to_db(&self.connection, &lamp_data)?;

        Ok(())
    }

    pub fn get_devices(&self) -> Result<Vec<LampData>, Box<dyn Error>> {
        Ok(get_devices_from_db(&self.connection)?)
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

fn create_lamp_data_table(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS LampData (
            id_lamp_data INTEGER PRIMARY KEY,
            id_device INTEGER,
            timestamp INTEGER,
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

fn add_device_to_db(connection: &Connection, lamp_data: &LampData) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "INSERT INTO devices (name, mac_address) VALUES (?1, ?2)",
        (&lamp_data.device_name, &lamp_data.device_mac)
    )?;

    Ok(())
}

fn get_devices_from_db(connection: &Connection) -> Result<Vec<LampData>, Box<dyn Error>> {
    let mut stmt = connection.prepare("SELECT * FROM devices")?;
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

fn add_lamp_data_to_db(connection: &Connection, lamp_data: &LampData) -> Result<(), Box<dyn Error>> {
    let device_id = get_device_id(connection, lamp_data)?;

    if device_id.is_none() {
        error!("Device not found in db! mac: {}, name: {}", &lamp_data.device_mac, &lamp_data.device_name);
        return Err(Box::new(DBHandlerError("Device not found in db".into())));
    }

    connection.execute(
        "INSERT INTO LampData (
            id_device,
            timestamp,
            illuminance, 
            voltage, 
            current, 
            power, 
            energy, 
            frequency, 
            power_factor
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &device_id.unwrap(),
            &lamp_data.timestamp,
            &lamp_data.illuminance,
            &lamp_data.voltage,
            &lamp_data.current,
            &lamp_data.power,
            &lamp_data.energy,
            &lamp_data.frequency,
            &lamp_data.power_factor
        )
    )?;

    Ok(())
}

fn get_device_id(connection: &Connection, lamp_data: &LampData) -> Result<Option<usize>, Box<dyn Error>> {
    let mut stmt = connection.prepare("SELECT id_device
                                                    FROM devices
                                                    WHERE mac_address = ?1"
                                                 )?;
    let mut rows = stmt.query_map([&lamp_data.device_mac], |row| row.get(0))?;

    let mut devices_id = Vec::new();
    for row in rows {
        devices_id.push(row?);
    }
    
    debug!("Device id vec: {:?}", devices_id);

    if let Some(device_id) = devices_id.first() {
        return Ok( Some(*device_id) );
    };

    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;

    fn connect_to_dummy_db() -> Result<Connection, Box<dyn Error>> {
        Ok(Connection::open_in_memory()?)
    }

    fn create_tables(connection: &Connection) -> Result<(), Box<dyn Error>> {
        create_devices_table(&connection)?;
        create_lamp_data_table(&connection)?;

        Ok(())
    }

    #[test]
    fn connect_to_database() -> Result<(), Box<dyn Error>> {
        DBHandler::new()?;

        Ok(())
    }

    #[test]
    fn creating_tables() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        create_tables(&connection)?;

        Ok(())
    }

    #[test]
    fn add_and_get_device() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let lamp_data = LampData::new();

        create_tables(&connection)?;
        add_device_to_db(&connection, &lamp_data)?;

        let devices = get_devices_from_db(&connection)?;

        assert_eq!(lamp_data, *devices.first().unwrap());

        Ok(())
    }

    #[test]
    fn add_lamp_data() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let lamp_data = LampData::new();

        create_tables(&connection)?;
        add_device_to_db(&connection, &lamp_data)?;
        add_lamp_data_to_db(&connection, &lamp_data)?;

        Ok(())
    }

    #[test]
    fn get_device_id_success() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let lamp_data = LampData::new();

        create_tables(&connection)?;
        add_device_to_db(&connection, &lamp_data)?;

        get_device_id(&connection, &lamp_data)?.expect("device not found");

        Ok(())
    }
}
