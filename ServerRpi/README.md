# ServerRpi


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
