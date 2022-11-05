pub mod device_type_utils {
    use crate::DeviceType;

    const LAMP_CONTROLLER: &str = "lamp controller";
    const UNKNOWN: &str = "unknown";

    use DeviceType::*;

    pub fn to_string(device_type: &DeviceType) -> String {
        String::from(match device_type {
            LampController => LAMP_CONTROLLER,
            UnknownDevice => UNKNOWN,
        })
    }

    pub fn from_string(string: &String) -> DeviceType {
        match string.as_str() {
            LAMP_CONTROLLER => LampController,
            _ => UnknownDevice,
        }
    }
}

pub mod measurement_type_utils {
    use crate::MeasurementType;

    const ILLUMINANCE: &str = "illuminance";
    const VOLTAGE: &str = "voltage";
    const CURRENT: &str = "current";
    const POWER: &str = "power";
    const ENERGY: &str = "energy";
    const FREQUENCY: &str = "frequency";
    const POWER_FACTOR: &str = "power factor";
    const UNKNOWN: &str = "unknown";

    use MeasurementType::*;

    pub fn to_string(measurement_type: &MeasurementType) -> String {
        String::from(match measurement_type {
            Illuminance => ILLUMINANCE,
            Voltage => VOLTAGE,
            Current => CURRENT,
            Power => POWER,
            Energy => ENERGY,
            Frequency => FREQUENCY,
            PowerFactor => POWER_FACTOR,
            UnknownMeasurment => UNKNOWN,
        })
    }

    pub fn from_string(string: &String) -> MeasurementType {
        match string.as_str() {
            ILLUMINANCE => Illuminance,
            VOLTAGE => Voltage,
            CURRENT => Current,
            POWER => Power,
            ENERGY => Energy,
            FREQUENCY => Frequency,
            POWER_FACTOR => PowerFactor,
            _ => UnknownMeasurment,
        }
    }
}
