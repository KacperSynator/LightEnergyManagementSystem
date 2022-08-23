#pragma once

#include <string>

#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>
#include <BLE2902.h>
#include <Arduino.h>

const auto kServiceUUID {"75b17eef-0276-4e5d-a97b-afc0eff7b4dd"};
const auto kRxUUID {"ab5ba0ce-027f-4d68-a362-82bb281e5884"};
const auto kTxUUID {"cea4c36a-1f89-4646-a368-704446233be2"};

const auto kLampDimPin{13};
class BLEConnection : public BLEServerCallbacks, public BLECharacteristicCallbacks {
  public:
    void onConnect(BLEServer* server) override;
    void onDisconnect(BLEServer* server) override;
    void onWrite(BLECharacteristic *characteristic) override;
    void Setup();
    void SendData(const std::string& data);

  private:
    BLEServer *server_{nullptr};
    BLECharacteristic *tx_characteristic_{nullptr};
    bool connected_{false};
};
