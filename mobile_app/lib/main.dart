import 'package:flutter/material.dart';
import 'dart:async';

import 'utils.dart';
import 'mobile_app.dart';
import 'widgets/devices_list.dart';
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
  final mobileApp =
      MobileApp(devicesStreamController, deviceNameChangeStreamController);

  @override
  void initState() {
    mobileApp.init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text("Light Energy Management System"),
        ),
        body: Column(
          children: [
            DevicesList(
              devicesStreamController: devicesStreamController,
              deviceNameChangeStreamController:
                  deviceNameChangeStreamController,
            )
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
