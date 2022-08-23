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
#include "pwm_handler.h"

const int kDimLedChannel{0};
const int kDimPin{13};
const int kDimPwmFreq{1000};
const int kDimPwmResolution{16};

class LampController {
  public:
    bool Setup();
    void Loop();

  private:
    lamp_controller_LampData lamp_data_;
    PwmHandler lamp_dim_{kDimLedChannel, kDimPin, kDimPwmFreq, kDimPwmResolution};
    BH1750 light_meter_;
    BLEConnection ble_connection_;
    PZEM004Tv30 pzem_{Serial2, 16, 17};
};
