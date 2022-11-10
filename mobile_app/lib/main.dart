import 'package:flutter/material.dart';

import 'mqtt_connection.dart';
import 'utils.dart';

void main() async {
  initLogger();
  final mqttConnection = MqttConnection(
      host: "test.mosquitto.org",
      clientId: "mobile_app",
      willMsg: "disconnected",
      keepAlive: 30);
  await mqttConnection.publish("mobile_app", "hello");
  await mqttConnection.subscribe(
      "mobile_app/#",
      (message) => logger.info(
          "MqttConnection: received msg: topiC: ${message.variableHeader!.topicName}\n\t ${message.payload}"));
  runApp(MobileApp());
}

class MobileApp extends StatefulWidget {
  @override
  State<MobileApp> createState() => _MobileAppState();
}

class _MobileAppState extends State<MobileApp> {
  @override
  Widget build(BuildContext constext) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text("Light Energy Management System"),
        ),
      ),
    );
  }
}
