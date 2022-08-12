#include <string>

#include <Arduino.h>
#include <Wire.h>
#include <BH1750.h>

#include "ble_connection.h"

BH1750 lightMeter;
BLEConnection ble_connection;

void setup(){

  Serial.begin(9600);

  // Initialize the I2C bus (BH1750 library doesn't do this automatically)
  // On esp8266 devices you can select SCL and SDA pins using Wire.begin(D4, D3);
  Wire.begin();

  lightMeter.begin();
  Serial.println(F("BH1750 Test"));
  ble_connection.Setup();
}

void loop() {

  float lux = lightMeter.readLightLevel();
  ble_connection.SendData("Light: " + std::to_string(lux) + " lx");
  ble_connection.LoopIteration();
  delay(1000);
  
}