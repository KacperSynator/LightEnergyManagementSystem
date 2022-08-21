#pragma once

#include <string>

#include <Arduino.h>
#include <Wire.h>
#include <BH1750.h>
#include <PZEM004Tv30.h>
#include <pb_common.h>
#include <pb.h>
#include <pb_encode.h>
#include "ble_connection.h"
#include "lamp_controller.pb.h"

class LampController {
  public:
    bool Setup();
    void Loop();

  private:
    lamp_controller_LampData lamp_data_;
    BH1750 light_meter_;
    BLEConnection ble_connection_;
    PZEM004Tv30 pzem_{Serial2, 16, 17};
};
