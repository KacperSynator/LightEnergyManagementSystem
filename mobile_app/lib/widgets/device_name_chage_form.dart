import 'package:flutter/material.dart';

class DeviceNameChangeForm extends StatelessWidget {
  final TextEditingController textController;
  final VoidCallback onSubmitted;

  const DeviceNameChangeForm({
    super.key,
    required this.textController,
    required this.onSubmitted,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.end,
      children: [
        TextField(
          decoration: const InputDecoration(labelText: "New name"),
          controller: textController,
          keyboardType: TextInputType.text,
          onSubmitted: (_) => onSubmitted(),
        ),
        TextButton(
          onPressed: onSubmitted,
          child: const Text("Save"),
        ),
      ],
    );
    ;
  }
}
