#include <Arduino.h>

#include "lamp_controller.h"

const auto kSerialBaud{115200};

LampController lamp_controller;

void setup() {
  Serial.begin(kSerialBaud);

  lamp_controller.Setup();
}

void loop() {
  lamp_controller.Loop();
}
