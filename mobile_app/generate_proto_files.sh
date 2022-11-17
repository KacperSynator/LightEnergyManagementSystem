#!/bin/bash

cp ../proto/light_energy_menagement_system.proto lib/proto
flutter pub run build_runner build
rm lib/proto/light_energy_menagement_system.proto
