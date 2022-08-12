#include "ble_connection.h"

void BLEConnection::onWrite(BLECharacteristic* characteristic) {
    std::string rx_value = characteristic->getValue();

    if (rx_value.empty()) return;

    Serial.println("*********");
    Serial.print("Received Value: ");
    Serial.print(rx_value.c_str());
    Serial.println();
    Serial.println("*********");
}

void BLEConnection::Setup() {
    BLEDevice::init("UART Service For ESP32");

    server_ = BLEDevice::createServer();
    server_->setCallbacks(this);

    auto service = server_->createService(kServiceUUID);

    tx_characteristic_ = service->createCharacteristic(kTxUUID, BLECharacteristic::PROPERTY_NOTIFY);
    tx_characteristic_->addDescriptor(new BLE2902());

    auto rx_characteristic = service->createCharacteristic(kRxUUID, BLECharacteristic::PROPERTY_WRITE);
    rx_characteristic->setCallbacks(this);

    service->start();
    server_->getAdvertising()->start();
    Serial.println("Waiting a client connection to notify...");
}

void BLEConnection::SendData(const std::string& data) {
    if (device_connected_) {
        tx_characteristic_->setValue(data);
        tx_characteristic_->notify();
        delay(10);
    }
}

void BLEConnection::LoopIteration() {
    Disconnecting();
    Connecting();
}

void BLEConnection::Disconnecting() {
    if (!device_connected_ && old_device_connected_) {
        delay(500);  // give the bluetooth stack the chance to get things ready
        server_->startAdvertising();
        Serial.println("start advertising");
        old_device_connected_ = device_connected_;
    }
}

void BLEConnection::Connecting() {
    if (device_connected_ && !old_device_connected_) {
        Serial.println("Connecting");
        old_device_connected_ = device_connected_;
    }
}
