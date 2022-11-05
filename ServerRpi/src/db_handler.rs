use crate::proto_utils::{device_type_utils, measurement_type_utils};
use crate::DataPacket;
use crate::Device;
use crate::DeviceMeasurments;
use crate::Measurement;
use crate::MeasurementType;

use log::{debug, error};
use protobuf::EnumOrUnknown;
use rusqlite::{Connection, ErrorCode, Result};
use std::error::Error;
use std::fmt;

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
        setup_db(&connection)?;

        Ok(Self { connection })
    }

    pub fn insert_data_packet(&self, data_packet: &DataPacket) -> Result<(), Box<dyn Error>> {
        add_device_to_db(&self.connection, &data_packet.device)?;
        add_device_measurements_to_db(
            &self.connection,
            &data_packet.device_measurements,
            &data_packet.device,
        )?;

        Ok(())
    }

    // pub fn get_all_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
    //     get_all_devices_from_db(&self.connection)
    // }
}

fn setup_db(connection: &Connection) -> Result<(), Box<dyn Error>> {
    create_devices_table(connection)?;
    create_channels_table(connection)?;
    create_measurements_table(connection)?;

    Ok(())
}

fn create_devices_table(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS Devices (
            id_device  INTEGER PRIMARY KEY,
            type TEXT NOT NULL,
            name TEXT,
            mac_address TEXT NOT NULL UNIQUE
        )",
        (),
    )?;

    Ok(())
}

fn create_measurements_table(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS Measurements (
            id_device INTEGER,
            id_channel INTEGER,
            timestamp DATETIME,
            value REAL,
            status TEXT,
            PRIMARY KEY(id_device, timestamp, id_channel),
            FOREIGN KEY(id_device) REFERENCES Devices(id_device),
            FOREIGN KEY(id_channel) REFERENCES Channels(id_channel)
        )",
        (),
    )?;

    Ok(())
}

fn create_channels_table(connection: &Connection) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS Channels (
            id_channel INTEGER PRIMARY KEY,
            id_device INTEGER,
            name TEXT UNIQUE,
            FOREIGN KEY(id_device) REFERENCES Devices(id_device)
        )",
        (),
    )?;

    Ok(())
}

fn add_device_to_db(connection: &Connection, device: &Device) -> Result<(), Box<dyn Error>> {
    let device_type = device_type_utils::to_string(&device.type_.enum_value_or_default());
    let res = connection.execute(
        "INSERT INTO devices (name, mac_address, type) VALUES (?1, ?2, ?3)",
        (&device.name, &device.mac, &device_type),
    );

    if let Err(e) = res {
        if e.sqlite_error_code().unwrap() == ErrorCode::ConstraintViolation {
            return Ok(());
        }

        return Err(Box::new(e));
    }

    Ok(())
}

fn add_device_measurements_to_db(
    connection: &Connection,
    device_measurements: &Vec<DeviceMeasurments>,
    device: &Device,
) -> Result<(), Box<dyn Error>> {
    let device_id = get_device_id(connection, device)?;

    for device_measurement in device_measurements.iter() {
        let timestamp = device_measurement.timestamp;
        for measurement in device_measurement.measurements.iter() {
            add_measurement_to_db(connection, measurement, timestamp, device_id)?;
        }
    }

    Ok(())
}

fn add_measurement_to_db(
    connection: &Connection,
    measurement: &Measurement,
    timestamp: u32,
    device_id: u64,
) -> Result<(), Box<dyn Error>> {
    let status = if measurement.value <= 0.0 {"invalid"} else {"valid"};
    add_channel_to_db(connection, &measurement.type_.enum_value_or_default(), device_id)?;
    let channel_id = get_channel_id(connection, &measurement.type_.enum_value_or_default())?;
    connection.execute(
        "INSERT INTO Measurements (
                                id_device,
                                id_channel,
                                timestamp,
                                value,
                                status
                            )
                            VALUES(?1, ?2, ?3, ?4, ?5)",
        (device_id, channel_id, timestamp, measurement.value, status),
    )?;

    Ok(())
}

fn add_channel_to_db(
    connection: &Connection,
    measurement_type: &MeasurementType,
    device_id: u64,
) -> Result<(), Box<dyn Error>> {
    let res = connection.execute(
        "INSERT INTO Channels (id_device, name) VALUES (?1, ?2)",
        (
            &device_id,
            measurement_type_utils::to_string(measurement_type),
        ),
    );

    if let Err(e) = res {
        if e.sqlite_error_code().unwrap() == ErrorCode::ConstraintViolation {
            return Ok(());
        }

        return Err(Box::new(e));
    }

    Ok(())
}

fn get_device_id(connection: &Connection, device: &Device) -> Result<u64, Box<dyn Error>> {
    let mut stmt = connection.prepare(
        "SELECT id_device
              FROM Devices
              WHERE mac_address = ?1",
    )?;

    Ok(stmt.query_row([&device.mac], |row| row.get::<_, u64>(0))?)
}

fn get_channel_id(connection: &Connection, measurement_type: &MeasurementType) -> Result<u64, Box<dyn Error>> {
    let mut stmt = connection.prepare(
        "SELECT id_channel
              FROM Channels
              WHERE name = ?1",
    )?;

    Ok(stmt.query_row([&measurement_type_utils::to_string(measurement_type)], |row| row.get::<_, u64>(0))?)
}

fn get_all_devices_from_db(connection: &Connection) -> Result<Vec<Device>, Box<dyn Error>> {
    let mut stmt = connection.prepare("SELECT name, mac_address, type FROM devices")?;
    let devices_iter = stmt.query_map([], |row| {
        let mut device = Device::new();
        device.name = row.get(0)?;
        device.mac = row.get(1)?;
        device.type_ = EnumOrUnknown::new(device_type_utils::from_string(&row.get::<_, String>(2)?));

        Ok(device)
    })?;

    let devices = devices_iter
        .map(|device| device.unwrap())
        .collect::<Vec<_>>();

    debug!("Devices received from db\n\t{:?}", devices);

    Ok(devices)
}

// fn get_device_lamp_data_before(
//     connection: &Connection,
//     device: &Device,
//     timestamp: u32,
// ) -> Result<Vec<LampData>, Box<dyn Error>> {
//     let device_id = get_device_id(connection, device)?;

//     if device_id.is_none() {
//         error!(
//             "Device not found in db! mac: {}, name: {}",
//             &device.mac, &device.name
//         );
//         return Err(Box::new(DBHandlerError("Device not found in db".into())));
//     }

//     let mut stmt = connection.prepare(
//         "SELECT id_channel, timestamp FROM LampData
//              WHERE id_device = ?1 AND timestamp <= ?2",
//     )?;
//     let iter_data = stmt.query_map((&device_id.unwrap(), &timestamp), |row| {
//         let id_channel: u64 = row.get(0)?;
//         let timestamp: u32 = row.get(1)?;

//         Ok((id_channel, timestamp))
//     })?;

//     let data = iter_data
//         .filter(|id_and_timestamp| id_and_timestamp.is_ok())
//         .map(|id_and_timestamp| {
//             let (id_channel, timestamp) = id_and_timestamp.unwrap();
//             let mut lamp_data =
//                 select_lamp_data_from_channels_by_id(connection, id_channel).unwrap();
//             lamp_data.timestamp = timestamp;

//             lamp_data
//         })
//         .collect::<Vec<_>>();

//     debug!(
//         "Lamp data for device {:?} before {} received from db\n\t{:?}",
//         device, timestamp, data
//     );

//     Ok(data)
// }

// fn get_device_lamp_data_after(
//     connection: &Connection,
//     device: &Device,
//     timestamp: u32,
// ) -> Result<Vec<LampData>, Box<dyn Error>> {
//     let devices_before = get_device_lamp_data_before(connection, device, timestamp)?;
//     let all_devices = get_device_lamp_data_before(connection, device, u32::MAX)?;
//     let devices_after = all_devices[devices_before.len()..].to_vec();

//     debug!("Devices after: {:?}", devices_after);

//     Ok(devices_after)
// }

// fn get_last_insert_rowid(connection: &Connection) -> Result<Option<u64>, Box<dyn Error>> {
//     let mut stmt = connection.prepare("SELECT last_insert_rowid()")?;
//     let rows = stmt.query_map([], |row| row.get(0))?;

//     let mut vec = Vec::new();
//     for row in rows {
//         vec.push(row?);
//     }

//     debug!("Last insert rowid in vec: {:?}", vec);

//     if let Some(last_rowid) = vec.first() {
//         return Ok(Some(*last_rowid));
//     };

//     Ok(None)
// }

// fn select_lamp_data_from_channels_by_id(
//     connection: &Connection,
//     channel_id: u64,
// ) -> Result<LampData, Box<dyn Error>> {
//     let mut stmt = connection.prepare(
//         "SELECT illuminance, voltage, current, power, energy, frequency, power_factor
//              FROM Channels WHERE id_channel = ?1",
//     )?;
//     let iter_lamp_data = stmt.query_map([channel_id], |row| {
//         let mut lamp_data = LampData::new();
//         lamp_data.illuminance = row.get(0)?;
//         lamp_data.voltage = row.get(1)?;
//         lamp_data.current = row.get(2)?;
//         lamp_data.power = row.get(3)?;
//         lamp_data.energy = row.get(4)?;
//         lamp_data.frequency = row.get(5)?;
//         lamp_data.power_factor = row.get(6)?;

//         Ok(lamp_data)
//     })?;

//     let mut lamp_data = iter_lamp_data
//         .filter(|lamp_data| lamp_data.is_ok())
//         .map(|lamp_data| lamp_data.unwrap())
//         .collect::<Vec<_>>();

//     if lamp_data.is_empty() {
//         return Err(Box::new(DBHandlerError(
//             "Lamp data not found in channels".into(),
//         )));
//     }

//     Ok(lamp_data.remove(0))
// }

// fn add_channel_to_db(connection: &Connection, lamp_data: &LampData) -> Result<(), Box<dyn Error>> {
//     connection.execute(
//         "INSERT INTO Channels (
//             illuminance,
//             voltage,
//             current,
//             power,
//             energy,
//             frequency,
//             power_factor
//         )
//         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//         (
//             &lamp_data.illuminance,
//             &lamp_data.voltage,
//             &lamp_data.current,
//             &lamp_data.power,
//             &lamp_data.energy,
//             &lamp_data.frequency,
//             &lamp_data.power_factor,
//         ),
//     )?;

//     Ok(())
// }

// fn add_lamp_data_to_db(
//     connection: &Connection,
//     lamp_data: &LampData,
//     device: &Device,
// ) -> Result<(), Box<dyn Error>> {
//     add_channel_to_db(connection, lamp_data)?;
//     let last_rowid_channel = get_last_insert_rowid(connection)?;

//     if last_rowid_channel.is_none() {
//         return Err(Box::new(DBHandlerError(
//             "Last inserted rowid not found".into(),
//         )));
//     }

//     let device_id = get_device_id(connection, device)?;

//     if device_id.is_none() {
//         return Err(Box::new(DBHandlerError(
//             format!(
//                 "Device not found in db mac: {}, name: {}",
//                 &device.mac, &device.name
//             )
//             .into(),
//         )));
//     }

//     let mut stmt = connection.prepare("SELECT id_channel FROM Channels WHERE rowid = ?1")?;
//     let id_channel = stmt.query_row([last_rowid_channel.unwrap()], |row| row.get::<_, u64>(0));

//     if id_channel.is_err() {
//         return Err(Box::new(DBHandlerError(
//             "Id channel not found by rowid".into(),
//         )));
//     }

//     connection.execute(
//         "INSERT INTO LampData (
//             id_device,
//             id_channel,
//             timestamp
//         )
//         VALUES (?1, ?2, ?3)",
//         (
//             &device_id.unwrap(),
//             &id_channel.unwrap(),
//             &lamp_data.timestamp,
//         ),
//     )?;

//     Ok(())
// }

// fn get_channels_id(connection: &Connection, device_id: u64) -> Result<Vec<u64>, Box<dyn Error>> {
//     let mut stmt = connection.prepare(
//         "SELECT id_channel
//               FROM LampData
//               WHERE id_device = ?1",
//     )?;

//     let rows = stmt.query_map([&device_id], |row| row.get(0))?;

//     let mut channels_id = Vec::new();
//     for row in rows {
//         channels_id.push(row?);
//     }

//     debug!("Channel id vec: {:?}", channels_id);

//     Ok(channels_id)
// }

#[cfg(test)]
mod test {
    use super::*;

    fn connect_to_dummy_db() -> Result<Connection, Box<dyn Error>> {
        // Ok(Connection::open_in_memory()?)
        Ok(Connection::open("test.db3")?)
    }

    #[test]
    fn connect_to_database() -> Result<(), Box<dyn Error>> {
        DBHandler::new()?;

        Ok(())
    }

    #[test]
    fn db_setup() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        setup_db(&connection)?;

        Ok(())
    }

    #[test]
    fn add_and_get_device() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let device = Device::new();

        setup_db(&connection)?;
        add_device_to_db(&connection, &Device::new())?;

        let devices = get_all_devices_from_db(&connection)?;

        assert_eq!(device, *devices.first().unwrap());

        Ok(())
    }

    #[test]
    fn add_same_device_twice() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let device = Device::new();

        setup_db(&connection)?;
        add_device_to_db(&connection, &device)?;
        add_device_to_db(&connection, &device)?;

        assert_eq!(get_all_devices_from_db(&connection)?.len(), 1);

        Ok(())
    }

    #[test]
    fn add_data_packet() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let data_packet = DataPacket::new();

        setup_db(&connection)?;
        add_device_to_db(&connection, &data_packet.device)?;
        add_device_measurements_to_db(&connection, &data_packet.device_measurements, &data_packet.device)?;

        Ok(())
    }

    #[test]
    fn get_device_id_success() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let data_packet = DataPacket::new();

        setup_db(&connection)?;
        add_device_to_db(&connection, &data_packet.device)?;

        get_device_id(&connection, &data_packet.device)?;

        Ok(())
    }

    // fn setup_device_lamp_data_before_and_after(
    // ) -> Result<(Connection, Device, LampData, LampData, u32), Box<dyn Error>> {
    //     let connection = connect_to_dummy_db()?;
    //     let device = Device::new();
    //     let timestamp: u32 = 1000;
    //     let lamp_data_before = LampData::new();
    //     let mut lamp_data_after = LampData::new();
    //     lamp_data_after.timestamp = timestamp + 1;

    //     setup_db(&connection)?;
    //     add_device_to_db(&connection, &device)?;
    //     add_lamp_data_to_db(&connection, &lamp_data_before, &device)?;
    //     add_lamp_data_to_db(&connection, &lamp_data_after, &device)?;

    //     Ok((
    //         connection,
    //         device,
    //         lamp_data_before,
    //         lamp_data_after,
    //         timestamp,
    //     ))
    // }

    // #[test]
    // fn get_device_lamp_data_before_success() -> Result<(), Box<dyn Error>> {
    //     let (connection, device, lamp_data_before, _, timestamp) =
    //         setup_device_lamp_data_before_and_after()?;

    //     let result = get_device_lamp_data_before(&connection, &device, timestamp)?;

    //     assert_eq!(result.len(), 1);
    //     assert_eq!(result.first().unwrap(), &lamp_data_before);

    //     Ok(())
    // }

    // #[test]
    // fn get_device_lamp_data_after_success() -> Result<(), Box<dyn Error>> {
    //     let (connection, device, _, lamp_data_after, timestamp) =
    //         setup_device_lamp_data_before_and_after()?;

    //     let result = get_device_lamp_data_after(&connection, &device, timestamp)?;

    //     assert_eq!(result.len(), 1);
    //     assert_eq!(result.first().unwrap(), &lamp_data_after);

    //     Ok(())
    // }
}
