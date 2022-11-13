# mobile_app

A new Flutter project.

## Getting Started

#### 1. Install flutter

#### 2. Install packages
```dart
flutter pub add mqtt_client logging
```
#### 3. (optional) Generate protobuf files
```dart
cp ../proto/light_energy_menagement_system.proto lib/proto
flutter pub run build_runner build
rm lib/proto/light_energy_menagement_system.proto
```