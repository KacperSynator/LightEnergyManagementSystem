#include <string>

#include <Arduino.h>
#include <Wire.h>
#include <BH1750.h>
#include <pb_common.h>
#include <pb.h>
#include <pb_encode.h>
#include <PZEM004Tv30.h>

#include "ble_connection.h"
#include "lamp_controller.pb.h"

BH1750 lightMeter;
BLEConnection ble_connection;
// PZEM004Tv30 pzem(Serial2, 16, 17);

void EncodeLampData(const lamp_controller_LampData& data);

void setup(){

  Serial.begin(115200);
  
  Wire.begin();

  lightMeter.begin();
  Serial.println(F("BH1750 Test"));
  ble_connection.Setup();

  // pzem.resetEnergy();
}

void loop() {

  float lux = lightMeter.readLightLevel();
  ble_connection.SendData("Light: " + std::to_string(lux) + " lx");

  lamp_controller_LampData lamp_data = lamp_controller_LampData_init_zero;
  lamp_data.name.arg = (void*)"Lamp 1";
  lamp_data.illuminance = lux;
  EncodeLampData(lamp_data);
  delay(1000);
  
}

void EncodeLampData(const lamp_controller_LampData& data) {
  uint8_t buffer[128];
  pb_ostream_t stream = pb_ostream_from_buffer(buffer, sizeof(buffer));
  Serial.println(data.illuminance);
  if (!pb_encode(&stream, lamp_controller_LampData_fields, &data)){
    Serial.println("failed to encode temp proto");
    return;
  }

  Serial.printf("Message length: %d\n", stream.bytes_written);

  for(int i = 0; i<stream.bytes_written; i++){
    Serial.printf("%c",buffer[i]);
  }
}
