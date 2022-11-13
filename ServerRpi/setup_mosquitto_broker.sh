#!/bin/bash

CONFIG_DIR_PATH=/etc/mosquitto/conf.d
CONFIG_FILENAME=server_rpi.conf

sudo echo "listener 1883" > $CONFIG_DIR_PATH/$CONFIG_FILENAME
sudo echo "allow_anonymous true" >> $CONFIG_DIR_PATH/$CONFIG_FILENAME
sudo kill $(ps aux |awk '/mosquitto/ {print $2}')
sudo systemctl stop mosquitto.service
sudo service mosquitto stop
sudo mosquitto -c $CONFIG_DIR_PATH/$CONFIG_FILENAME -d
