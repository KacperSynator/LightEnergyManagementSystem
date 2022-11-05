#include "lamp_controller.h"

namespace {
using DataPacket = light_energy_management_system_DataPacket;
using Measurment = light_energy_management_system_Measurement;
using DeviceMeasurments = light_energy_management_system_DeviceMeasurments;
using MeasurementType = light_energy_management_system_MeasurementType;
using Measurments = std::vector<Measurment>;

const auto Illuminance = light_energy_management_system_MeasurementType_Illuminance;
const auto Voltage = light_energy_management_system_MeasurementType_Voltage;
const auto Current = light_energy_management_system_MeasurementType_Current;
const auto Power = light_energy_management_system_MeasurementType_Power;
const auto Energy = light_energy_management_system_MeasurementType_Energy;
const auto Frequency = light_energy_management_system_MeasurementType_Frequency;
const auto PowerFactor = light_energy_management_system_MeasurementType_PowerFactor;

bool encode_string(pb_ostream_t* stream, const pb_field_t* field, void* const* arg) {
    const char* str = static_cast<const char*>(*arg);

    if (!pb_encode_tag_for_field(stream, field))
        return false;

    return pb_encode_string(stream, (uint8_t*)str, strlen(str));
}

bool encode_measurments(pb_ostream_t* stream, const pb_field_t* field, void* const* arg) {
    const Measurments measurments = *static_cast<Measurments*>(*arg);

    Serial.printf("encode_lamp_data: %f, %d\n", measurments.front().value, measurments.front().type);

    for (const auto msrt : measurments) {
        if (!pb_encode_tag(stream, PB_WT_STRING, field->tag))
            return false;
        
        if (!pb_encode_submessage(stream, light_energy_management_system_Measurement_fields, &msrt)) 
            return false;
    }

    return true;
}

bool encode_device_measurments(pb_ostream_t* stream, const pb_field_t* field, void* const* arg) {
    Serial.printf("encode_device_measurments:\n");
    
    if (!pb_encode_tag_for_field(stream, field))
        return false;
    
    return pb_encode_submessage(stream, light_energy_management_system_DeviceMeasurments_fields, *arg);
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

bool TryReadToLampData(const float& data, Measurments& measurments, const MeasurementType& type) {
    if (isnan(data)) {
        measurments.emplace_back(Measurment{0.0f, type});
        return false;
    }

    measurments.emplace_back(Measurment{data, type});
    return true;
}

bool ReadEnergyMeterData(Measurments& measurments, PZEM004Tv30& energy_meter) {
    bool succes = true;

    succes &= TryReadToLampData(energy_meter.voltage(), measurments, Voltage);
    succes &= TryReadToLampData(energy_meter.current(),measurments, Current);
    succes &= TryReadToLampData(energy_meter.power(), measurments, Power);
    succes &= TryReadToLampData(energy_meter.energy(), measurments, Energy);
    succes &= TryReadToLampData(energy_meter.frequency(), measurments, Frequency);
    succes &= TryReadToLampData(energy_meter.pf(), measurments, PowerFactor);

    return succes;
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

float CalculateDutyCycle(const int& threshold,
                         const float& current_duty_cycle,
                         const float& illuminance)
{
    float relative_max_lux = illuminance / (current_duty_cycle + 0.1);
    float new_duty_cycle = static_cast<float>(threshold) / (relative_max_lux + 0.1);

    if (new_duty_cycle > 1.0) new_duty_cycle = 1.0;
    if (new_duty_cycle < 0.0) new_duty_cycle = 0.0;

    return new_duty_cycle;
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

    Measurments measurments{};
    measurments.emplace_back(Measurment{light_meter_.readLightLevel(), Illuminance});

    // Serial.println(pzem_.readAddress(true), HEX);

    if (!ReadEnergyMeterData(measurments, pzem_)) {
        Serial.println("Failed to read energy meter data!");
    }

    float duty = CalculateDutyCycle(lux_threshold_, dim_duty_cycle_, measurments.front().value);
    Serial.printf("Current duty: %f\n", dim_duty_cycle_);
    Serial.printf("Calculated duty: %f\n", duty);
    Serial.printf("Illuminance: %f\n", measurments.front().value);
    
    if (duty < 0.1) {
        digitalWrite(kRelayPin, HIGH);
        lamp_dim_.DutyCycle(0.1f);
        dim_duty_cycle_ = 0.0f;
    } else {
        digitalWrite(kRelayPin, LOW);
        lamp_dim_.DutyCycle(duty);
        dim_duty_cycle_ = duty;
    }

    DeviceMeasurments device_measurments = light_energy_management_system_DeviceMeasurments_init_zero;
    device_measurments.measurements.arg = static_cast<void*>(&measurments);
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
