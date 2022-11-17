import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:mobile_app/proto/light_energy_management_system.pbjson.dart';
import 'dart:async';

import '../proto/light_energy_management_system.pb.dart';
import 'device_card.dart';
import 'device_name_chage_form.dart';
import '../utils.dart';

class DevicesList extends StatefulWidget {
  final StreamController<Devices> devicesStreamController;
  final StreamController<Device> deviceNameChangeStreamController;
  final StreamController<Device> deviceMeasurementsRequestStreamController;

  const DevicesList(
      {super.key,
      required this.devicesStreamController,
      required this.deviceNameChangeStreamController,
      required this.deviceMeasurementsRequestStreamController});

  @override
  State<DevicesList> createState() => _DevicesListState();
}

class _DevicesListState extends State<DevicesList> {
  List<Device> devices = [];
  final textController = TextEditingController();

  @override
  void initState() {
    widget.devicesStreamController.stream.listen((devices) {
      _updateDevices(devices);
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: ListView.builder(
        itemBuilder: (ctx, index) => DeviceCard(
          device: devices[index],
          onTap: _requestDeviceMeasurements,
          onLongPress: _startRequestDeviceNameChange,
        ),
        itemCount: devices.length,
        shrinkWrap: true,
      ),
    );
  }

  void _updateDevices(Devices devices) {
    setState(() {
      this.devices = devices.devices;
    });
  }

  void _startRequestDeviceNameChange(Device device) {
    showModalBottomSheet(
      context: context,
      builder: (ctx) {
        return DeviceNameChangeForm(
          textController: textController,
          onSubmitted: () => _onSubmitted(device),
        );
      },
    );
  }

  void _onSubmitted(Device device) {
    _requestDeviceNameChange(device);
    Navigator.pop(context);
  }

  void _requestDeviceNameChange(Device device) {
    if (textController.text.isEmpty) {
      logger.warning("DevicesList: empty name provided");
      return;
    }

    device.name = textController.text;
    widget.deviceNameChangeStreamController.sink.add(device);
  }

  void _requestDeviceMeasurements(Device device) {
    widget.deviceMeasurementsRequestStreamController.sink.add(device);
  }
}
