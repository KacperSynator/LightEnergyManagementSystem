import 'package:flutter/material.dart';

import '../proto/light_energy_management_system.pb.dart';

class DeviceCard extends StatelessWidget {
  final Device device;
  final Function onLongPress;
  final Function onTap;

  const DeviceCard(
      {super.key,
      required this.device,
      required this.onLongPress,
      required this.onTap});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: Card(
        child: ListTile(
          leading: _getDeviceIcon(device.type),
          title: Text(device.name),
          subtitle: Text(device.mac),
          onTap: () => onTap(device),
          onLongPress: () => onLongPress(device),
        ),
      ),
    );
  }

  Icon _getDeviceIcon(DeviceType type) {
    switch (type) {
      case DeviceType.LampController:
        return const Icon(Icons.light);
      default:
        return const Icon(Icons.device_unknown);
    }
  }
}