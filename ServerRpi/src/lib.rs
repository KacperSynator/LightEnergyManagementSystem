mod db_handler;
mod mqtt_connection;
mod proto_utils;
pub mod server_rpi;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use light_energy_management_system::{
    DataPacket, Device, DeviceMeasurements, DeviceType, Devices, Measurement, MeasurementStatus,
    MeasurementType, MqttCommand, MqttPayload,
};
