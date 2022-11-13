import 'package:flutter/material.dart';

import 'utils.dart';
import 'mobile_app.dart';

void main() {
  initLogger();
  runApp(MobileAppUi());
}

class MobileAppUi extends StatefulWidget {
  @override
  State<MobileAppUi> createState() => _MobileAppUiState();
}

class _MobileAppUiState extends State<MobileAppUi> {
  final mobileApp = MobileApp();

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
        body: Column(children: [
          ElevatedButton(
            onPressed: () async {
              mobileApp.requestGetAllDevices();
            },
            child: const Text("DeviceRequest"),
          ),
          ElevatedButton(
            onPressed: () async {
              mobileApp.requestGetAllDevices();
            },
            child: const Text("DeviceRequest"),
          ),
        ]),
      ),
    );
  }
}
