mod db_handler;
mod mqtt_connection;
pub mod server_rpi;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use light_energy_menagment_system::{DataPacket, Device, LampData};
