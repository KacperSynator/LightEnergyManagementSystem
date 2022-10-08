# LocalRpi


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