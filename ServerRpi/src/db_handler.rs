use crate::proto_utils::{device_type_utils, measurement_type_utils};
use crate::DataPacket;
use crate::Device;
use crate::DeviceMeasurments;
use crate::Measurement;
use crate::MeasurementType;

use log::debug;
use protobuf::{EnumOrUnknown, SpecialFields};
use rusqlite::{Connection, ErrorCode, Result};
use std::collections::HashMap;
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
        add_data_packet_to_db(&self.connection, data_packet)?;

        Ok(())
    }

    pub fn get_all_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        get_all_devices_from_db(&self.connection)
    }

    pub fn get_device_by_mac(&self, mac: &String) -> Result<Device, Box<dyn Error>> {
        get_device_by_mac(&self.connection, mac)
    }

    pub fn get_measurements_of_device_after(
        &self,
        device: &Device,
        timestamp: &u32,
    ) -> Result<Vec<DeviceMeasurments>, Box<dyn Error>> {
        get_measurements_of_device_after(&self.connection, device, timestamp)
    }

    pub fn get_measurements_of_device_until(
        &self,
        device: &Device,
        timestamp: &u32,
    ) -> Result<Vec<DeviceMeasurments>, Box<dyn Error>> {
        get_measurements_of_device_until(&self.connection, device, timestamp)
    }

    pub fn get_all_measurements_of_device(
        &self,
        device: &Device,
    ) -> Result<Vec<DeviceMeasurments>, Box<dyn Error>> {
        get_all_measurements_of_device(&self.connection, device)
    }
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

fn add_data_packet_to_db(
    connection: &Connection,
    data_packet: &DataPacket,
) -> Result<(), Box<dyn Error>> {
    add_device_to_db(connection, &data_packet.device)?;
    add_device_measurements_to_db(
        connection,
        &data_packet.device_measurements,
        &data_packet.device,
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
    let status = if measurement.value <= 0.0 {
        "invalid"
    } else {
        "valid"
    };
    add_channel_to_db(
        connection,
        &measurement.type_.enum_value_or_default(),
        device_id,
    )?;
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

fn get_device_by_mac(connection: &Connection, mac: &String) -> Result<Device, Box<dyn Error>> {
    let mut stmt = connection.prepare(
        "SELECT type, name
        FROM Devices
        WHERE mac_address = ?1",
    )?;

    Ok(stmt.query_row([mac], |row| {
        let mut device = Device::new();
        device.mac = mac.clone();
        device.type_ = EnumOrUnknown::new(device_type_utils::from_string(&row.get(0)?));
        device.name = row.get(1)?;

        Ok(device)
    })?)
}

fn get_device_id(connection: &Connection, device: &Device) -> Result<u64, Box<dyn Error>> {
    let mut stmt = connection.prepare(
        "SELECT id_device
              FROM Devices
              WHERE mac_address = ?1",
    )?;

    Ok(stmt.query_row([&device.mac], |row| row.get::<_, u64>(0))?)
}

fn get_channel_id(
    connection: &Connection,
    measurement_type: &MeasurementType,
) -> Result<u64, Box<dyn Error>> {
    let mut stmt = connection.prepare(
        "SELECT id_channel
              FROM Channels
              WHERE name = ?1",
    )?;

    Ok(stmt.query_row(
        [&measurement_type_utils::to_string(measurement_type)],
        |row| row.get::<_, u64>(0),
    )?)
}

fn get_all_devices_from_db(connection: &Connection) -> Result<Vec<Device>, Box<dyn Error>> {
    let mut stmt = connection.prepare("SELECT name, mac_address, type FROM devices")?;
    let devices_iter = stmt.query_map([], |row| {
        let mut device = Device::new();
        device.name = row.get(0)?;
        device.mac = row.get(1)?;
        device.type_ =
            EnumOrUnknown::new(device_type_utils::from_string(&row.get::<_, String>(2)?));

        Ok(device)
    })?;

    let devices = devices_iter
        .filter(|device| device.is_ok())
        .map(|device| device.unwrap())
        .collect::<Vec<_>>();

    debug!("Devices received from db\n\t{:?}", devices);

    Ok(devices)
}

fn get_all_measurements_of_device(
    connection: &Connection,
    device: &Device,
) -> Result<Vec<DeviceMeasurments>, Box<dyn Error>> {
    get_measurements_of_device_until(connection, device, &u32::MAX)
}

fn get_measurements_of_device_after(
    connection: &Connection,
    device: &Device,
    timestamp: &u32,
) -> Result<Vec<DeviceMeasurments>, Box<dyn Error>> {
    let measurements_until = get_measurements_of_device_until(connection, device, timestamp)?;
    let all_measurements = get_all_measurements_of_device(connection, device)?;
    let measurements_after = all_measurements[measurements_until.len()..].to_vec();

    debug!("Devices after: {:?}", measurements_after);

    Ok(measurements_after)
}

fn get_measurements_of_device_until(
    connection: &Connection,
    device: &Device,
    timestamp: &u32,
) -> Result<Vec<DeviceMeasurments>, Box<dyn Error>> {
    let mut stmt = connection.prepare(
        "SELECT 
            Channels.name,
            Measurements.timestamp,
            Measurements.value,
            Measurements.status
        FROM Measurements
        INNER JOIN Devices
        ON Measurements.id_device = ?1
        INNER JOIN Channels
        ON Measurements.id_channel = Channels.id_channel
        WHERE Measurements.timestamp <= ?2",
    )?;

    let mut timestamp_measurements: HashMap<u32, Vec<Measurement>> = HashMap::new();
    let mut rows = stmt.query((get_device_id(connection, device)?, &timestamp))?;
    while let Some(row) = rows.next()? {
        let mut measurement = Measurement::new();
        measurement.type_ = EnumOrUnknown::new(measurement_type_utils::from_string(&row.get(0)?));
        measurement.value = row.get(2)?;
        let timestamp = row.get::<_, u32>(1)?;

        if !timestamp_measurements.contains_key(&timestamp) {
            timestamp_measurements.insert(timestamp, Vec::new());
        }

        timestamp_measurements
            .get_mut(&timestamp)
            .unwrap()
            .push(measurement);
    }

    Ok(timestamp_measurements
        .drain()
        .map(|(timestamp, measurements)| DeviceMeasurments {
            timestamp,
            measurements,
            special_fields: SpecialFields::new(),
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod test {
    use crate::DeviceType;

    use super::*;
    use protobuf::{MessageField, SpecialFields};

    fn connect_to_dummy_db() -> Result<Connection, Box<dyn Error>> {
        Ok(Connection::open_in_memory()?)
        // Ok(Connection::open("test.db3")?)
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
    fn add_same_channel_twice() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let measurement_type = &MeasurementType::Illuminance;
        let device_id = 1;

        setup_db(&connection)?;
        add_channel_to_db(&connection, measurement_type, device_id)?;
        add_channel_to_db(&connection, measurement_type, device_id)?;

        Ok(())
    }

    fn create_data_packet(
        device_mac: String,
        device_name: String,
        device_type: DeviceType,
        device_measurements: Option<&DeviceMeasurments>,
    ) -> DataPacket {
        let device_measurements = if device_measurements.is_none() {
            Vec::new()
        } else {
            vec![device_measurements.unwrap().clone()]
        };
        DataPacket {
            device: MessageField::some(Device {
                mac: device_mac,
                name: device_name,
                type_: EnumOrUnknown::new(device_type),
                special_fields: SpecialFields::new(),
            }),
            device_measurements,
            special_fields: SpecialFields::new(),
        }
    }

    fn create_device_measurments(timestamp: u32) -> DeviceMeasurments {
        DeviceMeasurments {
            timestamp,
            measurements: Vec::new(),
            special_fields: SpecialFields::new(),
        }
    }

    fn push_measurement(
        device_measurments: &mut DeviceMeasurments,
        value: f32,
        measurement_type: MeasurementType,
    ) {
        device_measurments.measurements.push(Measurement {
            value,
            type_: EnumOrUnknown::new(measurement_type),
            special_fields: SpecialFields::new(),
        });
    }

    #[test]
    fn add_get_data_packet() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let mut device_measurement = create_device_measurments(10);
        push_measurement(&mut device_measurement, 5.0, MeasurementType::Illuminance);
        push_measurement(&mut device_measurement, 6.0, MeasurementType::Current);

        let data_packet = create_data_packet(
            "123".to_string(),
            "".to_string(),
            DeviceType::LampController,
            Some(&device_measurement),
        );

        setup_db(&connection)?;
        add_device_to_db(&connection, &data_packet.device)?;
        add_device_measurements_to_db(
            &connection,
            &data_packet.device_measurements,
            &data_packet.device,
        )?;

        let res = get_all_measurements_of_device(&connection, &data_packet.device)?;
        assert_eq!(res.len(), 1);
        assert_eq!(res.first().unwrap(), &device_measurement);

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

    #[test]
    fn get_device_by_mac_success() -> Result<(), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let mac = String::from("mac");
        let data_packet = create_data_packet(
            mac.clone(),
            "test".to_string(),
            DeviceType::LampController,
            None,
        );

        setup_db(&connection)?;
        add_data_packet_to_db(&connection, &data_packet)?;

        let res_device = get_device_by_mac(&connection, &mac)?;
        assert_eq!(res_device.mac, data_packet.device.mac);

        Ok(())
    }

    fn setup_get_measurements_of_device_until_after(
    ) -> Result<(Connection, DataPacket, DeviceMeasurments, u32), Box<dyn Error>> {
        let connection = connect_to_dummy_db()?;
        let timestamp = 1000;
        let mut device_measurement = create_device_measurments(timestamp.clone());
        push_measurement(&mut device_measurement, 5.0, MeasurementType::Illuminance);

        let data_packet = create_data_packet(
            "123".to_string(),
            "".to_string(),
            DeviceType::LampController,
            Some(&device_measurement),
        );

        setup_db(&connection)?;
        add_device_to_db(&connection, &data_packet.device)?;
        add_device_measurements_to_db(
            &connection,
            &data_packet.device_measurements,
            &data_packet.device,
        )?;

        Ok((connection, data_packet, device_measurement, timestamp))
    }

    #[test]
    fn get_measurements_of_device_until_success() -> Result<(), Box<dyn Error>> {
        let (connection, data_packet, device_measurement, timestamp) =
            setup_get_measurements_of_device_until_after()?;

        let result =
            get_measurements_of_device_until(&connection, &data_packet.device, &timestamp)?;

        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap(), &device_measurement);

        let result =
            get_measurements_of_device_until(&connection, &data_packet.device, &(timestamp - 1))?;

        assert!(result.is_empty());

        Ok(())
    }

    #[test]
    fn get_measurements_of_device_after_success() -> Result<(), Box<dyn Error>> {
        let (connection, data_packet, device_measurement, timestamp) =
            setup_get_measurements_of_device_until_after()?;

        let result =
            get_measurements_of_device_after(&connection, &data_packet.device, &(timestamp - 1))?;

        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap(), &device_measurement);

        let result =
            get_measurements_of_device_after(&connection, &data_packet.device, &timestamp)?;

        assert!(result.is_empty());

        Ok(())
    }
}
