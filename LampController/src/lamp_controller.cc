#include "lamp_controller.h"

namespace {
using DataPacket = light_energy_management_system_DataPacket;
using Measurement = light_energy_management_system_Measurement;
using DeviceMeasurements = light_energy_management_system_DeviceMeasurements;
using MeasurementType = light_energy_management_system_MeasurementType;
using Measurements = std::vector<Measurement>;

constexpr auto Illuminance = light_energy_management_system_MeasurementType_Illuminance;
constexpr auto Voltage = light_energy_management_system_MeasurementType_Voltage;
constexpr auto Current = light_energy_management_system_MeasurementType_Current;
constexpr auto Power = light_energy_management_system_MeasurementType_Power;
constexpr auto Energy = light_energy_management_system_MeasurementType_Energy;
constexpr auto Frequency = light_energy_management_system_MeasurementType_Frequency;
constexpr auto PowerFactor = light_energy_management_system_MeasurementType_PowerFactor;
constexpr auto Invalid = light_energy_management_system_MeasurementStatus_Invalid;
constexpr auto Valid = light_energy_management_system_MeasurementStatus_Valid;

bool encode_string(pb_ostream_t* stream, const pb_field_t* field, void* const* arg) {
    const char* str = static_cast<const char*>(*arg);

    if (!pb_encode_tag_for_field(stream, field))
        return false;

    return pb_encode_string(stream, (uint8_t*)str, strlen(str));
}

bool encode_measurments(pb_ostream_t* stream, const pb_field_t* field, void* const* arg) {
    const Measurements measurments = *static_cast<Measurements*>(*arg);

    // Serial.printf("encode_lamp_data: %f, %d\n", measurments.front().value, measurments.front().type);

    for (const auto msrt : measurments) {
        if (!pb_encode_tag(stream, PB_WT_STRING, field->tag))
            return false;
        
        if (!pb_encode_submessage(stream, light_energy_management_system_Measurement_fields, &msrt)) 
            return false;
    }

    return true;
}

bool encode_device_measurments(pb_ostream_t* stream, const pb_field_t* field, void* const* arg) {
    // Serial.printf("encode_device_measurments:\n");
    
    if (!pb_encode_tag_for_field(stream, field))
        return false;
    
    return pb_encode_submessage(stream, light_energy_management_system_DeviceMeasurements_fields, *arg);
}

const std::string EncodeDataPacket(const DataPacket& data) {
    uint8_t buffer[256];
    pb_ostream_t stream = pb_ostream_from_buffer(buffer, sizeof(buffer));

    if (!pb_encode(&stream, light_energy_management_system_DataPacket_fields, &data)) {
        Serial.println("failed to encode data packet");
        return "Encode failed!";
    }

    Serial.printf("Message length: %d\n", stream.bytes_written);
    std::string result {buffer, buffer + stream.bytes_written};

    for(int i = 0; i < stream.bytes_written; i++){
        Serial.printf("%c",buffer[i]);
    }
    Serial.println();
    
    // Serial.print("\nSize: ");
    // Serial.println(result.size());

    // for (const auto& c : result) {
    //     Serial.print(c);
    // }
    // Serial.println();

    return result;

}

bool ValidateMeasurement(const float& data, Measurements& measurements,
                         const MeasurementType& type) {
    if (isnan(data)) {
        measurements.emplace_back(Measurement{0.0f, type, Invalid});
        return false;
    }

    measurements.emplace_back(Measurement{data, type, Valid});
    return true;
}

bool ReadEnergyMeterData(Measurements& measurements, PZEM004Tv30& energy_meter) {
    bool success = true;

    success &= ValidateMeasurement(energy_meter.voltage(), measurements, Voltage);
    success &= ValidateMeasurement(energy_meter.current(),measurements, Current);
    success &= ValidateMeasurement(energy_meter.power(), measurements, Power);
    success &= ValidateMeasurement(energy_meter.energy(), measurements, Energy);
    success &= ValidateMeasurement(energy_meter.frequency(), measurements, Frequency);
    success &= ValidateMeasurement(energy_meter.pf(), measurements, PowerFactor);

    return success;
}

const char* GetMacAddress() {
    return strdup(WiFi.macAddress().c_str());
}

void SetupDevice(DataPacket& data_packet) {
    data_packet = light_energy_management_system_DataPacket_init_zero;
    data_packet.has_device = true;
    data_packet.device.type = light_energy_management_system_DeviceType_LampController;
    data_packet.device.mac.arg = (void*) GetMacAddress();
    data_packet.device.mac.funcs.encode = &encode_string;
}

void ControlLight(const int& lux_threshold, float& dim_duty_cycle,
                  const float& illuminance, const PwmHandler& lamp_dim)
{
    if (digitalRead(kPirPin) == LOW) {
        Serial.println("No movement detected");
        return;
    }

    if (abs(lux_threshold - illuminance) >= 5) {
        Serial.println("Current illuminace is OK");
        return;
    }

    if (lux_threshold > illuminance) {
        Serial.println("Illuminance too low");
        dim_duty_cycle += 0.01f;
    } else {
        Serial.println("Illuminance too high");
        dim_duty_cycle -= 0.01f;
    }

    Serial.printf("New duty: %f\n", dim_duty_cycle);
    Serial.printf("Illuminance: %f\n", illuminance);

    if (dim_duty_cycle > 1.0f) dim_duty_cycle = 1.0f;
    
    if (dim_duty_cycle < 0.1f) {
        dim_duty_cycle = 0.1f;
        digitalWrite(kRelayPin, HIGH);
    } else {
        digitalWrite(kRelayPin, LOW);
    }

    lamp_dim.DutyCycle(dim_duty_cycle);
}

}  // namespace


void LampController::Setup() {
    setup_status_.all_clear = (
        (setup_status_.wire = Wire.begin()) 
        && (setup_status_.light_meter = light_meter_.begin()) 
        && (setup_status_.lamp_dim = lamp_dim_.Setup()) 
        && (setup_status_.energy_meter = (pzem_.readAddress() != 0x00))
    );

    pinMode(kRelayPin, OUTPUT);
    pinMode(kPirPin, OUTPUT);
    
    ble_connection_.Setup();

    SetupDevice(data_packet_);

    pzem_.resetEnergy();
}

void LampController::Loop() {
    delay(5000);

    if (!setup_status_.all_clear) {
        Serial.println(setup_status_.str().c_str());
        // ble_connection_.SendData(setup_status_.str());
        // Setup();
        // return;
    }

    Measurements measurements{};
    const auto illuminance = light_meter_.readLightLevel();
    if (illuminance >= 0) {
        measurements.emplace_back(Measurement{illuminance, Illuminance, Valid});
    } else {
        measurements.emplace_back(Measurement{illuminance, Illuminance, Invalid});
    }

    ControlLight(lux_threshold_, dim_duty_cycle_, measurements.front().value, lamp_dim_);

    if (!ReadEnergyMeterData(measurements, pzem_)) {
        Serial.println("Failed to read energy meter data!");
    }

    DeviceMeasurements device_measurments = light_energy_management_system_DeviceMeasurements_init_zero;
    device_measurments.measurements.arg = static_cast<void*>(&measurements);
    device_measurments.measurements.funcs.encode = &encode_measurments;

    data_packet_.device_measurements.arg = static_cast<void*>(&device_measurments);
    data_packet_.device_measurements.funcs.encode = &encode_device_measurments;

    ble_connection_.SendData(EncodeDataPacket(data_packet_));
}

const std::string LampController::SetupStatus::str() {
    std::stringstream ss;
    ss << std::boolalpha 
       << "Initialisation failed SetupStatus:\n"
       << "\tWire: " << wire << "\n"
       << "\tEnergy meter: " << energy_meter << "\n"
       << "\tLight meter: " << light_meter << "\n"
       << "\tLamp dim:" << lamp_dim << "\n";

    return ss.str();
}
