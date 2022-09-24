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

void Reconnect(BLEServer* server) {
    delay(500);
    server->startAdvertising(); 
}

void BLEConnection::onConnect(BLEServer* server) {
    Serial.println("Connected");
    connected_ = true;
}

void BLEConnection::onDisconnect(BLEServer* server) {
    Serial.println("Disconnected");
    Reconnect(server);
    connected_ = false;
}

void BLEConnection::Setup() {
    BLEDevice::init(kDeviceName);

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
    if (connected_) {
        tx_characteristic_->setValue(data);
        tx_characteristic_->notify();
        delay(10);
    }
}
