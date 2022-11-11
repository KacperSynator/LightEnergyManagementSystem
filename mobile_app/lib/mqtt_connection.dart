import 'dart:async';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:mqtt_client/mqtt_server_client.dart';

import 'utils.dart';

class MqttConnection {
  late MqttServerClient _client;

  MqttConnection(
      {required String host,
      required String clientId,
      required String willMsg,
      required int keepAlive}) {
    _client = _buildClient(host, clientId, willMsg, keepAlive);
  }

  Future<bool> publish(String topic, String msg) async {
    if (!await checkConnection()) {
      return false;
    }

    final builder = MqttClientPayloadBuilder();
    builder.addString(msg);
    _client.publishMessage(topic, MqttQos.exactlyOnce, builder.payload!);

    logger.info("MqttConnection: published");

    return true;
  }

  Future<bool> subscribe(String topic, Function(List<MqttReceivedMessage<MqttMessage>>) callback) async {
    if (!await checkConnection()) {
      return false;
    }
    _client.updates!.listen(callback);
    return null != _client.subscribe(topic, MqttQos.exactlyOnce);
  }

  Future<bool> checkConnection() async {
    if (!isConnected()) {
      return await tryConnect();
    }
    return true;
  }

  bool isConnected() {
    return _client.connectionStatus!.state == MqttConnectionState.connected;
  }

  Future<bool> tryConnect() async {
    try {
      await _client.connect();
    } on Exception catch (e) {
      logger.shout('MqttConnection: when connecting, client exception: $e');
      _client.disconnect();
      return false;
    }
    return true;
  }

  static MqttServerClient _buildClient(
      String host, String clientId, String willMsg, int keepAlive) {
    final client = MqttServerClient(host, '');

    client.setProtocolV311();
    client.logging(on: false);
    client.keepAlivePeriod = keepAlive;
    client.onDisconnected = onDisconnected;
    client.onSubscribed = onSubscribed;
    client.connectionMessage = _buildConnectionMsg(clientId, willMsg);

    return client;
  }

  static MqttConnectMessage _buildConnectionMsg(
      String clientId, String willMsg) {
    return MqttConnectMessage()
        .withClientIdentifier(clientId)
        .withWillTopic("will")
        .withWillMessage(willMsg)
        .startClean() // Non persistent session for testing
        .withWillQos(MqttQos.atLeastOnce);
  }

  static void onSubscribed(String topic) {
    logger.info('MqttConnection: subscribed to topic: $topic');
  }

  static void onDisconnected() {
    logger.warning('MqttConnection: disconnected from server');
  }
}
