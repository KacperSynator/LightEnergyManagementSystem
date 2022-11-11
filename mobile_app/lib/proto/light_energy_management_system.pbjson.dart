///
//  Generated code. Do not modify.
//  source: light_energy_management_system.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
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
@$core.Deprecated('Use deviceMeasurmentsDescriptor instead')
const DeviceMeasurments$json = const {
  '1': 'DeviceMeasurments',
  '2': const [
    const {'1': 'timestamp', '3': 1, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'measurements', '3': 2, '4': 3, '5': 11, '6': '.light_energy_management_system.Measurement', '10': 'measurements'},
  ],
};

/// Descriptor for `DeviceMeasurments`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceMeasurmentsDescriptor = $convert.base64Decode('ChFEZXZpY2VNZWFzdXJtZW50cxIcCgl0aW1lc3RhbXAYASABKARSCXRpbWVzdGFtcBJPCgxtZWFzdXJlbWVudHMYAiADKAsyKy5saWdodF9lbmVyZ3lfbWFuYWdlbWVudF9zeXN0ZW0uTWVhc3VyZW1lbnRSDG1lYXN1cmVtZW50cw==');
@$core.Deprecated('Use dataPacketDescriptor instead')
const DataPacket$json = const {
  '1': 'DataPacket',
  '2': const [
    const {'1': 'device', '3': 1, '4': 1, '5': 11, '6': '.light_energy_management_system.Device', '10': 'device'},
    const {'1': 'device_measurements', '3': 2, '4': 3, '5': 11, '6': '.light_energy_management_system.DeviceMeasurments', '10': 'deviceMeasurements'},
  ],
};

/// Descriptor for `DataPacket`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dataPacketDescriptor = $convert.base64Decode('CgpEYXRhUGFja2V0Ej4KBmRldmljZRgBIAEoCzImLmxpZ2h0X2VuZXJneV9tYW5hZ2VtZW50X3N5c3RlbS5EZXZpY2VSBmRldmljZRJiChNkZXZpY2VfbWVhc3VyZW1lbnRzGAIgAygLMjEubGlnaHRfZW5lcmd5X21hbmFnZW1lbnRfc3lzdGVtLkRldmljZU1lYXN1cm1lbnRzUhJkZXZpY2VNZWFzdXJlbWVudHM=');
