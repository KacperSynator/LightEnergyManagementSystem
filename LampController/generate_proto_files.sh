#!/bin/bash

FILE_NAME=light_energy_menagment_system

protoc -I=.. --nanopb_out=. ./../proto/$FILE_NAME.proto
mkdir include/proto
mv proto/$FILE_NAME.pb.c src/
mv proto/$FILE_NAME.pb.h include/proto
rm -r proto
