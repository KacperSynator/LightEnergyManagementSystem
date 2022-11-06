mod ble_connection;
pub mod local_rpi;
mod mqtt_connection;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use light_energy_management_system::{DataPacket, DeviceMeasurments};
