# LocalRpi
Responsible for gathering data from LampControllers using BLE and sending it to ServerRpi via cellular network using LTE module. Data is send via MQTT protocol. After receiving the data from LampController LocalRpi adds current timestamp to data. Communication with LTE module is carried out with the help of the AT commands. Implemented on RPI 4B board.

## BLE
Scans for surrounding devices and connects to them based on the Tx charasteristic name, gathers data from characteristics disconencts and looks for another device. Can also send data directly to devices Rx characteristics. Implemented using [btleplug](https://github.com/deviceplug/btleplug) library with help of [tokio](https://tokio.rs/) library for rust asynchronous runtime.

## Cellular MQTT
Uses [A7670E LTE Cat-1 HAT](https://www.waveshare.com/a7670e-cat-1-hat.htm) for connecting to cellular network. LocalRpi communicates with Hat using AT commands to publish and subscribe to MQTT topics.

## Protobuf
Protobuf files are generated during cargo build process and are put in `target` directory. For more information about generation process of proto files look at [build.rs](build.rs) or [rust-protobuf docs](https://docs.rs/protobuf-codegen/latest/protobuf_codegen/).

## Getting started
#### 1. Install dependencies
```bash
sudo apt-get install build-essential bluez bluetooth pi-bluetooth libbluetooth-dev protobuf-compiler curl
```

#### 2. Install rust
```bash
curl https://sh.rustup.rs -sSf | sh
```

#### 3. Enable bluez for pi user
* open `bluetooth.conf` file using text editor

    ```bash
    sudo nano /etc/dbus-1/system.d/bluetooth.conf
    ```
* add following lines before `</busconfig>` tag
    ```bash
    <policy user="pi">
        <allow send_destination="org.bluez"/>
        <allow send_interface="org.bluez.Agent1"/>
        <allow send_interface="org.bluez.GattCharacteristic1"/>
        <allow send_interface="org.bluez.GattDescriptor1"/>
        <allow send_interface="org.freedesktop.DBus.ObjectManager"/>
        <allow send_interface="org.freedesktop.DBus.Properties"/>
    </policy>
    ```

#### 4. Build and run 
```bash
RUST_LOG=info cargo run
```

#### 5. Run tests (Optional)
```bash
cargo test
```

## Roadmap
- [x] Protobuf
- [x] BLE
    - [x] Receive data
    - [x] Send data
    - [x] Tests
- [ ] LTE module
    - [ ] AT commands
    - [ ] publish/subscribe using MQTT
    - [ ] Tests
- [ ] Integrate and test components
- [ ] Update README
