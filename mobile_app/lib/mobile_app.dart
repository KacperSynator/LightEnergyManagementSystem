import 'package:mqtt_client/mqtt_client.dart' hide MqttPayload;
import 'package:protobuf/protobuf.dart';
import 'dart:async';

import 'proto/light_energy_management_system.pb.dart';
import 'utils.dart';
import 'mqtt_connection.dart';

class MobileApp {
  final mqttConnection = MqttConnection(
      // host: "test.mosquitto.org",
      host: "192.168.1.223",
      clientId: "mobile_app",
      willMsg: "disconnected",
      keepAlive: 30);

  final String uniqueId = getUniqueId();
  final StreamController<Devices> devicesStreamController;
  final StreamController<Device> deviceNameChangeStreamController;
  final StreamController<Device> deviceMeasurementsRequestStreamController;
  final StreamController<DataPacket> deviceMeasurementsStreamController;

  MobileApp(
      {required this.devicesStreamController,
      required this.deviceNameChangeStreamController,
      required this.deviceMeasurementsStreamController,
      required this.deviceMeasurementsRequestStreamController}) {
    deviceNameChangeStreamController.stream.listen((device) async {
      await requestChangeDeviceName(device);
    });

    deviceMeasurementsRequestStreamController.stream.listen((device) async {
      await requestDeviceMeasurements(device);
    });
  }

  Future<bool> init() async {
    logger.info("MobileApp::init: uniqueId: $uniqueId");

    return await mqttConnection.subscribe("d/$uniqueId/#", _messageParser);
  }

  Future<bool> requestGetAllDevices() async {
    final mqttPayload = MqttPayload(command: MqttCommand.GetAllDevices);
    final encodedMqttPayload = mqttPayload.writeToBuffer();
    return await _publish(
        "u/$uniqueId", String.fromCharCodes(encodedMqttPayload));
  }

  Future<bool> requestChangeDeviceName(Device device) async {
    final mqttPayload = MqttPayload(
        command: MqttCommand.ChangeDeviceName, msg: [device.writeToBuffer()]);
    final encodedMqttPayload = mqttPayload.writeToBuffer();
    return await _publish(
        "u/$uniqueId", String.fromCharCodes(encodedMqttPayload));
  }

  Future<bool> requestDeviceMeasurements(Device device) async {
    final mqttPayload = MqttPayload(
        command: MqttCommand.GetDeviceMeasurements,
        msg: [device.writeToBuffer()]);
    final encodedMqttPayload = mqttPayload.writeToBuffer();
    return await _publish(
        "u/$uniqueId", String.fromCharCodes(encodedMqttPayload));
  }

  Future<bool> _publish(String topic, String msg) async {
    logger.info("MobileApp::publish: send msg to topic: $topic");
    return await mqttConnection.publish(topic, msg);
  }

  void _messageParser(List<MqttReceivedMessage<MqttMessage>> messageList) {
    final topic = messageList[0].topic;
    final message = messageList[0].payload as MqttPublishMessage;
    final payload = message.payload.message;

    logger.info("MqttConnection: received msg: topiC: $topic\n\t $payload");

    MqttPayload mqttPayload;

    try {
      mqttPayload = MqttPayload.fromBuffer(payload);
    } on InvalidProtocolBufferException {
      logger.shout("MqttConnection: failed to parse protobuf");
      return;
    }

    logger.info("MqttConnection: parsed protobuf dataPacket: $mqttPayload");

    switch (mqttPayload.command) {
      case MqttCommand.GetAllDevices:
        {
          _handleGetAllDevices(mqttPayload);
        }
        break;
      case MqttCommand.ChangeDeviceName:
        {
          _handleChangeDeviceName(mqttPayload);
        }
        break;
      case MqttCommand.GetDeviceMeasurements:
        {
          _handleDeviceMeasurements(mqttPayload);
        }
        break;
      case MqttCommand.GetDeviceMeasurementsAfter:
        {
          _handleDeviceMeasurements(mqttPayload);
        }
        break;
      case MqttCommand.GetDeviceMeasurementsBefore:
        {
          _handleDeviceMeasurements(mqttPayload);
        }
        break;
      default:
        {
          logger.warning(
              "MqttMsgParser: invalid command: ${mqttPayload.command}");
        }
        break;
    }
  }

  void _handleGetAllDevices(MqttPayload mqttPayload) {
    if (mqttPayload.msg.length != 1) {
      logger.shout("handleGetAllDevices: mqttPayload.msg is not of size 1");
      return;
    }

    final devices = Devices.fromBuffer(mqttPayload.msg[0]);
    logger.info("handleGetAllDevices: parsed devices: $devices");
    devicesStreamController.sink.add(devices);
  }

  void _handleChangeDeviceName(MqttPayload mqttPayload) {
    if (mqttPayload.msg.length != 1) {
      logger.shout("handleChangeDeviceName: mqttPayload.msg is not of size 1");
      return;
    }

    final response = String.fromCharCodes(mqttPayload.msg[0]);
    logger.info("handleChangeDeviceName: parsed response: $response");
    requestGetAllDevices();
  }

  void _handleDeviceMeasurements(MqttPayload mqttPayload) {
    if (mqttPayload.msg.length != 1) {
      logger
          .shout("handleDeviceMeasurements: mqttPayload.msg is not of size 1");
      return;
    }

    final dataPacket = DataPacket.fromBuffer(mqttPayload.msg[0]);
    logger.info("handleDeviceMeasurements: parsed data packet: $dataPacket");
    deviceMeasurementsStreamController.sink.add(dataPacket);
  }
}
