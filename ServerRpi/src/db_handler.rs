use rusqlite::{Connection, Result, ErrorCode};
use log::{error, debug};
use std::fmt;
use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use light_energy_menagment_system::{DataPacket, Device, LampData};

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

    pub fn insert_data_packet(&self, data_packet: &DataPacket) -> Result<(), Box<dyn Error>> {
        add_device_to_db(&self.connection, &data_packet.device)?;
        add_lamp_data_to_db(&self.connection, &data_packet.lamp_data, &data_packet.device)?;

        Ok(())
    }

    pub fn get_all_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        get_all_devices_from_db(&self.connection)
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
            mac_address  TEXT NOT NULL UNIQUE
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

fn add_device_to_db(connection: &Connection, device: &Device) -> Result<(), Box<dyn Error>> {
    let res = connection.execute(
        "INSERT INTO devices (name, mac_address) VALUES (?1, ?2)",
        (&device.name, &device.mac)
    );
    
    if let Err(e) = res {
        if e.sqlite_error_code().unwrap() == ErrorCode::ConstraintViolation {
            return Ok(());
        }

        return Err(Box::new(e));
    }

    Ok(())
}

fn get_all_devices_from_db(connection: &Connection) -> Result<Vec<Device>, Box<dyn Error>> {
    let mut stmt = connection.prepare("SELECT name, mac_address FROM devices")?;
    let devices_iter = stmt.query_map([], |row| {
        let mut device = Device::new();
        device.name = row.get(0)?;
        device.mac = row.get(1)?;

        Ok(device)
    })?;

    let devices = devices_iter
            .map(|device| { device.unwrap() })
            .collect::<Vec<_>>();

    debug!("Devices received from db\n\t{:?}", devices);

    Ok( devices )
}

fn get_device_lamp_data_before(connection: &Connection, device: &Device, timestamp: usize) -> Result<Vec<LampData>, Box<dyn Error>> {
    let device_id = get_device_id(connection, &device)?;

    if device_id.is_none() {
        error!("Device not found in db! mac: {}, name: {}", &device.mac, &device.name);
        return Err(Box::new(DBHandlerError("Device not found in db".into())));
    }

    let mut stmt = connection.prepare("SELECT * FROM LampData WHERE id_device = ?1 AND timestamp <= ?2")?;
    let iter_lamp_data = stmt.query_map((&device_id, &timestamp), |row| {
        let mut lamp_data = LampData::new();
        lamp_data.timestamp = row.get(2)?;
        lamp_data.illuminance = row.get(3)?;
        lamp_data.voltage = row.get(4)?;
        lamp_data.power = row.get(5)?;
        lamp_data.energy = row.get(6)?;
        lamp_data.frequency = row.get(7)?;
        lamp_data.power_factor = row.get(8)?;
        
        Ok(lamp_data)
    })?;

    let data = iter_lamp_data
            .map(|lamp_data| { lamp_data.unwrap() })
            .collect::<Vec<_>>();

    debug!("Lamp data for device {:?} before {} received from db\n\t{:?}", device, timestamp, data);

    Ok( data )
}

fn get_device_lamp_data_after(connection: &Connection, device: &Device, timestamp: usize) -> Result<Vec<LampData>, Box<dyn Error>> {
    let devices_before = get_device_lamp_data_before(connection, device, timestamp)?;
    let all_devices = get_device_lamp_data_before(connection, device, usize::MAX)?;

    Ok(all_devices[devices_before.len()-1..].to_vec())
}

fn add_lamp_data_to_db(connection: &Connection, lamp_data: &LampData, device: &Device) -> Result<(), Box<dyn Error>> {
    let device_id = get_device_id(connection, &device)?;

    if device_id.is_none() {
        error!("Device not found in db! mac: {}, name: {}", &device.mac, &device.name);
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

fn get_device_id(connection: &Connection, device: &Device) -> Result<Option<usize>, Box<dyn Error>> {
    let mut stmt = connection.prepare("SELECT id_device
                                                    FROM devices
                                                    WHERE mac_address = ?1"
                                                 )?;
    let rows = stmt.query_map([&device.mac], |row| row.get(0))?;

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
        let device = Device::new();

        create_tables(&connection)?;
        add_device_to_db(&connection, &Device::new())?;

        let devices = get_all_devices_from_db(&connection)?;

        assert_eq!(device, *devices.first().unwrap());

        Ok(())
    }

    #[test]
    fn add_same_device_twice() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let device = Device::new();

        create_tables(&connection)?;
        add_device_to_db(&connection, &device)?;
        add_device_to_db(&connection, &device)?;

        assert_eq!(get_all_devices_from_db(&connection)?.len(), 1);

        Ok(())
    }

    #[test]
    fn add_lamp_data() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let data_packet = DataPacket::new();

        create_tables(&connection)?;
        add_device_to_db(&connection, &data_packet.device)?;
        add_lamp_data_to_db(&connection, &data_packet.lamp_data, &data_packet.device)?;

        Ok(())
    }

    #[test]
    fn get_device_id_success() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let data_packet = DataPacket::new();

        create_tables(&connection)?;
        add_device_to_db(&connection, &data_packet.device)?;

        get_device_id(&connection, &data_packet.device)?.expect("device not found");

        Ok(())
    }

    fn setup_device_lamp_data_before_and_after() -> Result<(Connection, Device, LampData, LampData, usize), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let device = Device::new();
        let timestamp: u32 = 1000;
        let lamp_data_before = LampData::new();
        let mut lamp_data_after = LampData::new();
        lamp_data_after.timestamp = timestamp + 1;

        create_tables(&connection)?;
        add_device_to_db(&connection, &device)?;
        add_lamp_data_to_db(&connection, &lamp_data_before, &device)?;
        add_lamp_data_to_db(&connection, &lamp_data_after, &device)?;

        Ok ((connection, device, lamp_data_before, lamp_data_after, timestamp as usize))
    }

    #[test]
    fn get_device_lamp_data_before_success() -> Result<(), Box<dyn Error>> {
        let (connection, device, lamp_data_before, _, timestamp)  = setup_device_lamp_data_before_and_after()?;

        let result = get_device_lamp_data_before(&connection,  &device, timestamp)?;
        
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap(), &lamp_data_before);

        Ok(())
    }

    #[test]
    fn get_device_lamp_data_after_success() -> Result<(), Box<dyn Error>> {
        let (connection, device, _, lamp_data_after, timestamp)  = setup_device_lamp_data_before_and_after()?;

        let result = get_device_lamp_data_after(&connection,  &device, timestamp)?;
        
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap(), &lamp_data_after);

        Ok(())
    }
}
