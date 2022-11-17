import 'package:flutter/material.dart';
import 'dart:async';

import 'utils.dart';
import 'mobile_app.dart';
import 'widgets/devices_list.dart';
import 'widgets/device_measurements_graph.dart';
import 'proto/light_energy_management_system.pb.dart';

void main() {
  initLogger();
  runApp(const HomePage());
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  static final devicesStreamController = StreamController<Devices>();
  static final deviceNameChangeStreamController = StreamController<Device>();
  static final deviceMeasurementsStreamController =
      StreamController<DataPacket>();
  static final deviceMeasurementsRequestStreamController =
      StreamController<Device>();

  final mobileApp = MobileApp(
      devicesStreamController: devicesStreamController,
      deviceNameChangeStreamController: deviceNameChangeStreamController,
      deviceMeasurementsStreamController: deviceMeasurementsStreamController,
      deviceMeasurementsRequestStreamController:
          deviceMeasurementsRequestStreamController);

  @override
  void initState() {
    mobileApp.init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        brightness: Brightness.dark,
      ),
      home: Scaffold(
        appBar: AppBar(
          title: const Text(
            "Light Energy Management System",
            style: TextStyle(
              color: Color(0xff23b6e6),
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        body: Column(
          children: [
            DeviceMeasurementsGraph(
              deviceMeasurementsStreamController:
                  deviceMeasurementsStreamController,
            ),
            DevicesList(
              devicesStreamController: devicesStreamController,
              deviceNameChangeStreamController:
                  deviceNameChangeStreamController,
              deviceMeasurementsRequestStreamController:
                  deviceMeasurementsRequestStreamController,
            ),
          ],
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: () async {
            mobileApp.requestGetAllDevices();
          },
          child: const Icon(Icons.sensors),
        ),
      ),
    );
  }
}
