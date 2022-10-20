# ServerRpi
Responsible for receivng data data from LocalRpi using MQTT. Stores data in database implemented using sqlite3. Receives data requests from MobileApp using MQTT then fetches data from database and sends it to MobileApp. Hosts mosquitto broker for MQTT. Implemented on RPI 4B board.

## Database
Uses [SQLite](https://www.sqlite.org/index.html) as a database engine. Database is directly created and handled from Rust code using [rusqlite](https://github.com/rusqlite/rusqlite).

### Table Diagram
![image](https://user-images.githubusercontent.com/62207289/197052059-2a33685d-2b9b-4265-9637-56ac8fb21f2b.png)

## MQTT
Is implemented using [eclipse paho.mqtt.rust](https://github.com/eclipse/paho.mqtt.rust). Uses [AsynClient](https://docs.rs/paho-mqtt/0.7.1/paho_mqtt/async_client/index.html) for subscribing and publishing with help of [tokio](https://tokio.rs/) for rust asynchronous runtime. Subscribing and parsing messages is implemented using message callback. [Mosquitto](https://mosquitto.org/) broker is being hosted directly on RPI.

## Protobuf
Protobuf files are generated during cargo build process and are put in `target` directory. For more information about generation process of proto files look at [build.rs](build.rs) or [rust-protobuf docs](https://docs.rs/protobuf-codegen/latest/protobuf_codegen/).

## Getting started

#### 1. Install dependencies
```bash
sudo apt-get install build-essential curl git sqlite3 protobuf-compiler cmake libssl-dev mosquitto
```

#### 2. Install rust
```bash
curl https://sh.rustup.rs -sSf | sh
```

#### 3. Build and run 
```bash
RUST_LOG=info cargo run
```

## Roadmap
- [x] Protobuf
- [ ] Implement database
    - [x] Implement CREATE TABLES for devices and lamp_data
    - [x] Implement SELECT/INSERT requests for devices and lamp_data
    - [ ] Implement SELECT requests for lamp_data depending on device_name, timestamp
    - [ ] Unit tests for all free functions
- [ ] Implement MQTT
    - [x] Mosquitto broker
    - [x] find and add MQTT library
    - [x] Handler for some MQTT library 
    - [ ] specify topics and payloads
    - [ ] Unit tests for all free functions
- [ ] Integrate database with MQTT
- [ ] update README
