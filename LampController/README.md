# LampController
Responsible for controlling led module. Gathers data about power consumption and light intensity using energy meter and light sensor then uses 10V PWM to dim the ligth. Can turn off/on light using relay based on peaople presence that is detected using PIR motion sensor. Measured sensor data is send to LocalRpi via BLE (bluetooth low energy), it acts as a server and sends data when LocalRpi asks for it. Implemented on ESP32 devkit board.

## Energy meter
Uses (PZEM-004t v3)(https://innovatorsguru.com/pzem-004t-v3/) for garhering data: voltage, current, power, energy, frequency and power factor. Commenicates via ModbusRTU interface. For handling this device [PZEM-004T-v30](https://github.com/mandulaj/PZEM-004T-v30) library is used.

## Light sensor
Uses [BH1750](https://www.instructables.com/BH1750-Digital-Light-Sensor/) digital sensor with $I^2C$ bus interface. For handling this device [BH1750](https://www.arduino.cc/reference/en/libraries/bh1750/) library is used.

## Motion detector
Uses [PIR HC-SR501](https://pl.aliexpress.com/item/1005004590066028.html?_randl_currency=PLN&_randl_shipto=PL&src=google&src=google&albch=shopping&acnt=494-037-6276&slnk=&plac=&mtctp=&albbt=Google_7_shopping&albagn=888888&isSmbAutoCall=false&needSmbHouyi=false&albcp=12824521326&albag=128205084664&trgt=1656642000201&crea=pl1005004590066028&netw=u&device=c&albpg=1656642000201&albpd=pl1005004590066028&aff_fcid=9b1cf1dd5e99490796fb7089450ef3d1-1666301188297-00947-UneMJZVf&aff_fsk=UneMJZVf&aff_platform=aaf&sk=UneMJZVf&aff_trace_key=9b1cf1dd5e99490796fb7089450ef3d1-1666301188297-00947-UneMJZVf&terminal_id=aa3c6fdbcada49bc91894f6653546c17&afSmartRedirect=y) sensor and digital input pin to read if motion was detected.

## Protobuf
Uses [nanopb](https://jpa.kapsi.fi/nanopb/) C library. Files are generated manually using protoc and must be put in `include` and `src` directories, for convenience use [generate_proto_files.sh](generate_proto_files.sh) script.

### Generate protobuf files
```bash
chmod 777 generate_proto_files.sh
generate_proto_files.sh
```

## Getting Started
Use [PlatformioIO](https://platformio.org/) to build and install code to board. For convenience use [VSCode plugin](https://platformio.org/install/ide?install=vscode).

## Roadmap
- [x] Light sensor
- [x] Protobuf encoding/generation
- [ ] Energy meter
    - [x] Software
    - [ ] Connect and test hardware
- [ ] LED driver
    - [ ] Connect PWM and Relay
    - [ ] Implement automatic dimming
- [ ] Implement BLE
    - [x] Sending
    - [x] Receiving
    - [ ] Unit/Integration tests
- [ ] Integrate and test all components
