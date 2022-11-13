///
//  Generated code. Do not modify.
//  source: light_energy_management_system.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use mqttCommandDescriptor instead')
const MqttCommand$json = const {
  '1': 'MqttCommand',
  '2': const [
    const {'1': 'UnknownCommand', '2': 0},
    const {'1': 'HandleDataPacket', '2': 1},
    const {'1': 'GetAllDevices', '2': 2},
    const {'1': 'GetDeviceMeasurements', '2': 3},
    const {'1': 'GetDeviceMeasurementsBefore', '2': 4},
    const {'1': 'GetDeviceMeasurementsAfter', '2': 5},
    const {'1': 'ChangeDeviceName', '2': 6},
  ],
};

/// Descriptor for `MqttCommand`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List mqttCommandDescriptor = $convert.base64Decode('CgtNcXR0Q29tbWFuZBISCg5Vbmtub3duQ29tbWFuZBAAEhQKEEhhbmRsZURhdGFQYWNrZXQQARIRCg1HZXRBbGxEZXZpY2VzEAISGQoVR2V0RGV2aWNlTWVhc3VyZW1lbnRzEAMSHwobR2V0RGV2aWNlTWVhc3VyZW1lbnRzQmVmb3JlEAQSHgoaR2V0RGV2aWNlTWVhc3VyZW1lbnRzQWZ0ZXIQBRIUChBDaGFuZ2VEZXZpY2VOYW1lEAY=');
@$core.Deprecated('Use deviceTypeDescriptor instead')
const DeviceType$json = const {
  '1': 'DeviceType',
  '2': const [
    const {'1': 'UnknownDevice', '2': 0},
    const {'1': 'LampController', '2': 1},
  ],
};

/// Descriptor for `DeviceType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List deviceTypeDescriptor = $convert.base64Decode('CgpEZXZpY2VUeXBlEhEKDVVua25vd25EZXZpY2UQABISCg5MYW1wQ29udHJvbGxlchAB');
@$core.Deprecated('Use measurementTypeDescriptor instead')
const MeasurementType$json = const {
  '1': 'MeasurementType',
  '2': const [
    const {'1': 'UnknownMeasurment', '2': 0},
    const {'1': 'Illuminance', '2': 1},
    const {'1': 'Voltage', '2': 2},
    const {'1': 'Current', '2': 3},
    const {'1': 'Power', '2': 4},
    const {'1': 'Energy', '2': 5},
    const {'1': 'Frequency', '2': 6},
    const {'1': 'PowerFactor', '2': 7},
  ],
};

/// Descriptor for `MeasurementType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List measurementTypeDescriptor = $convert.base64Decode('Cg9NZWFzdXJlbWVudFR5cGUSFQoRVW5rbm93bk1lYXN1cm1lbnQQABIPCgtJbGx1bWluYW5jZRABEgsKB1ZvbHRhZ2UQAhILCgdDdXJyZW50EAMSCQoFUG93ZXIQBBIKCgZFbmVyZ3kQBRINCglGcmVxdWVuY3kQBhIPCgtQb3dlckZhY3RvchAH');
@$core.Deprecated('Use measurementStatusDescriptor instead')
const MeasurementStatus$json = const {
  '1': 'MeasurementStatus',
  '2': const [
    const {'1': 'Invalid', '2': 0},
    const {'1': 'Valid', '2': 1},
  ],
};

/// Descriptor for `MeasurementStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List measurementStatusDescriptor = $convert.base64Decode('ChFNZWFzdXJlbWVudFN0YXR1cxILCgdJbnZhbGlkEAASCQoFVmFsaWQQAQ==');
@$core.Deprecated('Use mqttPayloadDescriptor instead')
const MqttPayload$json = const {
  '1': 'MqttPayload',
  '2': const [
    const {'1': 'command', '3': 1, '4': 1, '5': 14, '6': '.light_energy_management_system.MqttCommand', '10': 'command'},
    const {'1': 'msg', '3': 2, '4': 3, '5': 12, '10': 'msg'},
  ],
};

/// Descriptor for `MqttPayload`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mqttPayloadDescriptor = $convert.base64Decode('CgtNcXR0UGF5bG9hZBJFCgdjb21tYW5kGAEgASgOMisubGlnaHRfZW5lcmd5X21hbmFnZW1lbnRfc3lzdGVtLk1xdHRDb21tYW5kUgdjb21tYW5kEhAKA21zZxgCIAMoDFIDbXNn');
@$core.Deprecated('Use deviceDescriptor instead')
const Device$json = const {
  '1': 'Device',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'mac', '3': 2, '4': 1, '5': 9, '10': 'mac'},
    const {'1': 'type', '3': 3, '4': 1, '5': 14, '6': '.light_energy_management_system.DeviceType', '10': 'type'},
  ],
};

/// Descriptor for `Device`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceDescriptor = $convert.base64Decode('CgZEZXZpY2USEgoEbmFtZRgBIAEoCVIEbmFtZRIQCgNtYWMYAiABKAlSA21hYxI+CgR0eXBlGAMgASgOMioubGlnaHRfZW5lcmd5X21hbmFnZW1lbnRfc3lzdGVtLkRldmljZVR5cGVSBHR5cGU=');
@$core.Deprecated('Use devicesDescriptor instead')
const Devices$json = const {
  '1': 'Devices',
  '2': const [
    const {'1': 'devices', '3': 1, '4': 3, '5': 11, '6': '.light_energy_management_system.Device', '10': 'devices'},
  ],
};

/// Descriptor for `Devices`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List devicesDescriptor = $convert.base64Decode('CgdEZXZpY2VzEkAKB2RldmljZXMYASADKAsyJi5saWdodF9lbmVyZ3lfbWFuYWdlbWVudF9zeXN0ZW0uRGV2aWNlUgdkZXZpY2Vz');
@$core.Deprecated('Use measurementDescriptor instead')
const Measurement$json = const {
  '1': 'Measurement',
  '2': const [
    const {'1': 'value', '3': 1, '4': 1, '5': 2, '10': 'value'},
    const {'1': 'type', '3': 2, '4': 1, '5': 14, '6': '.light_energy_management_system.MeasurementType', '10': 'type'},
    const {'1': 'status', '3': 3, '4': 1, '5': 14, '6': '.light_energy_management_system.MeasurementStatus', '10': 'status'},
  ],
};

/// Descriptor for `Measurement`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List measurementDescriptor = $convert.base64Decode('CgtNZWFzdXJlbWVudBIUCgV2YWx1ZRgBIAEoAlIFdmFsdWUSQwoEdHlwZRgCIAEoDjIvLmxpZ2h0X2VuZXJneV9tYW5hZ2VtZW50X3N5c3RlbS5NZWFzdXJlbWVudFR5cGVSBHR5cGUSSQoGc3RhdHVzGAMgASgOMjEubGlnaHRfZW5lcmd5X21hbmFnZW1lbnRfc3lzdGVtLk1lYXN1cmVtZW50U3RhdHVzUgZzdGF0dXM=');
@$core.Deprecated('Use deviceMeasurementsDescriptor instead')
const DeviceMeasurements$json = const {
  '1': 'DeviceMeasurements',
  '2': const [
    const {'1': 'timestamp', '3': 1, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'measurements', '3': 2, '4': 3, '5': 11, '6': '.light_energy_management_system.Measurement', '10': 'measurements'},
  ],
};

/// Descriptor for `DeviceMeasurements`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceMeasurementsDescriptor = $convert.base64Decode('ChJEZXZpY2VNZWFzdXJlbWVudHMSHAoJdGltZXN0YW1wGAEgASgEUgl0aW1lc3RhbXASTwoMbWVhc3VyZW1lbnRzGAIgAygLMisubGlnaHRfZW5lcmd5X21hbmFnZW1lbnRfc3lzdGVtLk1lYXN1cmVtZW50UgxtZWFzdXJlbWVudHM=');
@$core.Deprecated('Use dataPacketDescriptor instead')
const DataPacket$json = const {
  '1': 'DataPacket',
  '2': const [
    const {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.light_energy_management_system.Device', '10': 'device'},
    const {'1': 'device_measurements', '3': 2, '4': 3, '5': 11, '6': '.light_energy_management_system.DeviceMeasurements', '10': 'deviceMeasurements'},
  ],
};

/// Descriptor for `DataPacket`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataPacketDescriptor = $convert.base64Decode('CgpEYXRhUGFja2V0Ej4KBmRldmljZRgBIAEoCzImLmxpZ2h0X2VuZXJneV9tYW5hZ2VtZW50X3N5c3RlbS5EZXZpY2VSBmRldmljZRJjChNkZXZpY2VfbWVhc3VyZW1lbnRzGAIgAygLMjIubGlnaHRfZW5lcmd5X21hbmFnZW1lbnRfc3lzdGVtLkRldmljZU1lYXN1cmVtZW50c1ISZGV2aWNlTWVhc3VyZW1lbnRz');
