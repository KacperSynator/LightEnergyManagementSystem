# LightEnergyMenagmentSystem
The main goal is to gather data about power consumption by led lamps and then reduce that consumption by automated control of the lamp intensity. This project uses four different devices and IoT technologies to accomplish the above goal.

## Devices diagram
![image](https://user-images.githubusercontent.com/62207289/196780558-e45c3029-8b80-4b55-bc26-72ceb113174a.png)


| **Ref** |                             **Device**                                     |                               **Short Description**                            |
|:-------:|:--------------------------------------------------------------------------:|:------------------------------------------------------------------------------:|
|    0    |  [RPI 4B](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/)    |  Hosts database, mqtt broker                                                   |
|    1    |  [RPI 4B](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/)    |  Communicates with ESPs and server                                             |
|    2    |  [A7670E Cat-1 HAT](https://www.waveshare.com/a7670e-cat-1-hat.htm)        |  Provides internet connection to RPI and sends data/receives data from server  |
|    3    |  [NodeMCU-32 ESP32](https://esphome.io/devices/nodemcu_esp32.html)         |  Controlls sensors and light                                                   |
|    4    |      [PZEM-004T](https://aliexpress.com/item/4000330631886.html?spm=a2g0o.productlist.0.0.120c68dcRCsh14&algo_pvid=76bcbc46-381d-43a0-8567-daad81c0f7a6&algo_exp_id=76bcbc46-381d-43a0-8567-daad81c0f7a6-0&pdp_ext_f=%7B%22sku_id%22%3A%2210000001354084576%22%7D&pdp_npi=2%40dis%21PLN%2138.87%2128.77%21%21%2111.7%21%21%402100bde716662065654408715e5167%2110000001354084576%21sea&curPageLogUid=s0awogAPkbAZ)                                                        |          Measures energy consumption of lamp                                   |
|    5    |       [BH1750](https://aliexpress.com/item/1005004572545808.html?spm=a2g0o.productlist.0.0.7ddc4944vcF3tL&algo_pvid=6a9f13e8-8668-4b42-8cfd-72389552b79d&aem_p4p_detail=20221019121017598158602130320000601709&algo_exp_id=6a9f13e8-8668-4b42-8cfd-72389552b79d-1&pdp_ext_f=%7B%22sku_id%22%3A%2212000029674376515%22%7D&pdp_npi=2%40dis%21PLN%216.72%216.32%21%21%21%21%21%402100bde716662066176112206e5167%2112000029674376515%21sea&curPageLogUid=t5pxmKx1mDLH&ad_pvid=20221019121017598158602130320000601709_2&gatewayAdapt=glo2pol) |              Measures intensity of light                              |
|    6    | [GLP DIM driver](https://glpower.eu/en/product/gpf-25d/)                   |                       Driver of LED module                                     |
|    7    |    [PIR HC-SR501](https://pl.aliexpress.com/item/1005001621794785.html?spm=a2g0o.productlist.0.0.425c433aVTuQxn&ad_pvid=20221019121311198919320356880010768156_1&s=p)  |               Detects presence of people                              |
|    10   | Flutter mobile app                                                         | App for remote control and data presentation for user                          |


## [LampController](LampController)
Responsible for controlling led module. Gathers data about power consumption and light intensity using energy meter and light sensor then uses 10V PWM to dim the ligth. Can turn off/on light using relay based on peaople presence that is detected using PIR motion sensor. Measured sensor data is send to LocalRpi via BLE (bluetooth low energy), it acts as a server and sends data when LocalRpi asks for it.  

**Written in C++ with proto files generated for C.  
Used board: ESP32 programmed using PlatformIO.**

## [LocalRPI](LocalRpi)
Responsible for gathering data from LampControllers using BLE and sending it to ServerRpi via cellular network using LTE module. Data is send via MQTT protocol. After receiving the data from LampController LocalRpi adds current timestamp to data. Communication with LTE module is carried out with the help of the AT commands.

**Written in Rust  
Used board: RPI 4B**

## [ServerRPI](ServerRpi)
Responsible for receivng data data from LocalRpi using MQTT. Stores data in database implemented using sqlite3. Receives data requests from MobileApp using MQTT then fetches data from database and sends it to MobileApp. Hosts mosquitto broker for MQTT.

**Written in Rust  
Used board: RPI 4B**

## [MobileApp](MobileApp)
Responsible for presenting data for user. Used to customize names of devices. Data is presented as graphs for periods specified by user (last month, all time). Communicates with ServerRpi using MQTT.

**Written in Flutter**

## [Protobuf](proto)
For data serialization [protocal buffers](https://developers.google.com/protocol-buffers) are used. It was chosen becauese it is fast and easy to use and it generates code (yaay less typing). Current project state uses 3 messages:  
* Device - name, mac address of device  
* LampData - sensor data and timestamp
* DataPacket - above 2 in one message  

### Proto file
```proto3
syntax = "proto3";

package light_energy_management_system;

enum DeviceType {
    UnknownDevice = 0;
    LampController = 1;
}

message Device {
    string name = 1;
    string mac = 2;
    DeviceType type = 3;
}

enum MeasurementType {
    UnknownMeasurment = 0;
    Illuminance = 1;
    Voltage = 2;
    Current = 3;
    Power = 4;
    Energy = 5;
    Frequency = 6;
    PowerFactor = 7;
}

enum MeasurementStatus {
    Invalid = 0;
    Valid = 1;
}

message Measurement {
    float value = 1;
    MeasurementType type = 2;
    MeasurementStatus status = 3;
}

message DeviceMeasurments {
    uint64 timestamp = 1;
    repeated Measurement measurements = 2;
}

message DataPacket {
    Device device = 1;
    repeated DeviceMeasurments device_measurements = 2;
}
```
