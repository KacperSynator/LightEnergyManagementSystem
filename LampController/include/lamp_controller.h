#pragma once

#include <string>
#include <sstream>

#include <Arduino.h>
#include <WiFi.h>
#include <Wire.h>
#include <BH1750.h>
#include <PZEM004Tv30.h>
#include <pb_common.h>
#include <pb.h>
#include <pb_encode.h>

#include "ble_connection.h"
#include "proto/light_energy_menagment_system.pb.h"
#include "pwm_handler.h"

const int kDimLedChannel{0};
const int kDimPin{14};
const int kDimPwmFreq{1000};
const int kDimPwmResolution{16};
const int kMicroSecToSecFactor{1000000};
const int kRelayPin{26};
const int kPirPin{27};
const int kDefaultLuxThreshold{200};
const auto kDeviceName{"LampController"};

class LampController {
  public:
    void Setup();
    void Loop();

  private:
    using DataPacket = light_energy_menagment_system_DataPacket;

    struct SetupStatus {
      bool light_meter{false};
      bool energy_meter{false};
      bool wire{false};
      bool lamp_dim{false};
      bool all_clear{false};

      const std::string str();

    } setup_status_;

    float dim_duty_cycle_{};
    int lux_threshold_{kDefaultLuxThreshold};

    DataPacket data_packet_;
    PwmHandler lamp_dim_{kDimLedChannel, kDimPin, kDimPwmFreq, kDimPwmResolution};
    BH1750 light_meter_;
    BLEConnection ble_connection_;
    PZEM004Tv30 pzem_{Serial2, 16, 17};
};
