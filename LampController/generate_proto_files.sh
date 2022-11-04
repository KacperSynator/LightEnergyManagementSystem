#!/bin/bash

FILE_NAME=light_energy_management_system

protoc -I=.. --nanopb_out=. ./../proto/$FILE_NAME.proto
mv proto/$FILE_NAME.pb.c src/
rm -r include/proto
mv proto/ include/
