import 'package:flutter_test/flutter_test.dart';

import 'package:mobile_app/mqtt_connection.dart';

void main() {
  const topic = "mobile_app_test";
  const payload = "test";
  test("MqttConnection subscribe and publish", () async {
    final mqttConnection = MqttConnection(
        host: "test.mosquitto.org",
        clientId: "mobile_app_test",
        keepAlive: 5,
        willMsg: "mobile app test disconnected");
    expect(
        await mqttConnection.subscribe(topic, (msgList) {
          final msg = msgList[0];
          expect(msg.topic, topic);
        }),
        true);
    expect(await mqttConnection.publish(topic, payload), true);
  });
}
