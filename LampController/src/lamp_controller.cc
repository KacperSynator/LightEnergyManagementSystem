#include "lamp_controller.h"

namespace {

bool encode_string(pb_ostream_t* stream, const pb_field_t* field, void* const* arg)
{
    const char* str = (const char*)(*arg);

    if (!pb_encode_tag_for_field(stream, field))
        return false;

    return pb_encode_string(stream, (uint8_t*)str, strlen(str));
}

const std::string EncodeLampData(const lamp_controller_LampData& data) {
    uint8_t buffer[128];
    pb_ostream_t stream = pb_ostream_from_buffer(buffer, sizeof(buffer));

    if (!pb_encode(&stream, lamp_controller_LampData_fields, &data)){
        Serial.println("failed to encode temp proto");
        return "Encode failed!";
    }

    Serial.printf("Message length: %d\n", stream.bytes_written);
    std::string result {buffer, buffer + stream.bytes_written};

    for(int i = 0; i < stream.bytes_written; i++){
        Serial.printf("%c",buffer[i]);
    }
    
    Serial.print("\nSize: ");
    Serial.println(result.size());

    for (const auto& c : result) {
        Serial.print(c);
    }
    Serial.println();

    return result;
}

}  // namespace



bool LampController::Setup() {
    if (!Wire.begin()) {
        Serial.println("Wire begin failed!");
        return false;
    }
    
    if (!light_meter_.begin()) {
        Serial.println("Light meter begin failed!");
        return false;
    }

    ble_connection_.Setup();

    lamp_data_ = lamp_controller_LampData_init_zero;
    lamp_data_.name.arg = (void*)"LampController";
    lamp_data_.name.funcs.encode = &encode_string;

    pzem_.resetEnergy();

    return true;
}



void LampController::Loop() {
    lamp_data_.illuminance = light_meter_.readLightLevel();
    lamp_data_.voltage = pzem_.voltage();
    lamp_data_.current = pzem_.current();
    lamp_data_.power = pzem_.power();
    lamp_data_.energy = pzem_.energy();
    lamp_data_.frequency = pzem_.frequency();
    lamp_data_.power_factor = pzem_.pf();
    ble_connection_.SendData(EncodeLampData(lamp_data_));
    delay(1000);
}
