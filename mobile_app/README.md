# mobile_app
Responsible for presenting data for user. Used to customize names of devices. Data is presented as graphs for periods specified by user (last month, all time). Communicates with ServerRpi using MQTT.

Home screen            |  Change device name form
:-------------------------:|:-------------------------:
![image](https://user-images.githubusercontent.com/62207289/206870289-1d2782b6-2185-4616-92fe-43cc0a059abc.png)  |  ![image](https://user-images.githubusercontent.com/62207289/206870335-040b0d58-d0cb-42d4-a4f1-f8b7771d1b23.png)



## Protobuf
are generated manually using [protoc_builder](https://github.com/pikaju/dart-protoc-builder) and [build_runner](https://pub.dev/packages/build_runner). Files can be generated using bash script [generate_proto_files.sh](generate_proto_files.sh).

## Mqtt
Is implemented using [mqtt_client](https://pub.dev/packages/mqtt_client) package for asynchronous calls [dart_async](https://api.flutter.dev/flutter/dart-async/dart-async-library.html) is used.

## Graph plotting
is implemented using [fl_chart](https://pub.dev/packages/fl_chart).

## Getting Started
#### 1. Install flutter
Follow instructions from [flutter docs](https://docs.flutter.dev/get-started/instal)

#### 2. Install packages (not sure if needed)
```dart
flutter pub add mqtt_client logging
```

#### 3. Run
```dart
flutter run
```
#### 4. (optional) Generate protobuf files
```bash
# Linux only
./generate_proto_files.sh

# Window (run commands manually)
cp ../proto/light_energy_menagement_system.proto lib/proto
flutter pub run build_runner build
rm lib/proto/light_energy_menagement_system.proto
```
