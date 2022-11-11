import 'package:mqtt_client/mqtt_client.dart';
import 'package:protobuf/protobuf.dart';

import 'proto/light_energy_management_system.pb.dart';
import 'utils.dart';
import 'mqtt_connection.dart';

class MobileApp {
  final mqttConnection = MqttConnection(
      // host: "test.mosquitto.org",
      host: "192.168.1.109",
      clientId: "mobile_app",
      willMsg: "disconnected",
      keepAlive: 30);

  String uniqueId = getUniqueId();

  MobileApp();

  Future<bool> init() async {
    logger.info("MobileApp::init: uniqueId: $uniqueId");

    return await mqttConnection.subscribe("d/$uniqueId/#", _messageParser);
  }

  Future<bool> requestDeviceData() async {
    return await publish("u/$uniqueId/device_data", "0");
  }

  Future<bool> publish(String topic, String msg) async {
    logger.info("MobileApp::publish: send msg to topic: $topic");
    return await mqttConnection.publish(topic, msg);
  }

  void _messageParser(List<MqttReceivedMessage<MqttMessage>> messageList) {
    final topic = messageList[0].topic;
    final message = messageList[0].payload as MqttPublishMessage;
    final payload = message.payload.message;

    logger.info(
        "MqttConnection: received msg: topiC: $topic\n\t $payload");

    DataPacket dataPacket;
    try {
      dataPacket = DataPacket.fromBuffer(payload);
    } on InvalidProtocolBufferException {
      logger.shout("MqttConnection: failed to parse protobuf");
      return;
    }

    logger.info("MqttConnection: parsed protobuf dataPacket: $dataPacket");
  }
}
