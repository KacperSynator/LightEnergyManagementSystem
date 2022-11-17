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
        elevation: 3,
        shadowColor: const Color(0xff02d39a),
        color: const Color(0xff232d37),
        child: ListTile(
          leading: _getDeviceIcon(device.type),
          title: Text(
            device.name,
            style: const TextStyle(
              color: Color(0xff02d39a),
              fontWeight: FontWeight.bold,
              fontSize: 16,
            ),
          ),
          subtitle: Text(
            device.mac,
            style: const TextStyle(
              color: Color(0xff67727d),
              fontWeight: FontWeight.bold,
              fontSize: 14,
            ),
          ),
          onTap: () => onTap(device),
          onLongPress: () => onLongPress(device),
        ),
      ),
    );
  }

  Icon _getDeviceIcon(DeviceType type) {
    const color = Color(0xff23b6e6);
    switch (type) {
      case DeviceType.LampController:
        return const Icon(
          Icons.light,
          color: color,
        );
      default:
        return const Icon(
          Icons.device_unknown,
          color: color,
        );
    }
  }
}
