///
//  Generated code. Do not modify.
//  source: light_energy_management_system.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class DeviceType extends $pb.ProtobufEnum {
  static const DeviceType UnknownDevice = DeviceType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UnknownDevice');
  static const DeviceType LampController = DeviceType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LampController');

  static const $core.List<DeviceType> values = <DeviceType> [
    UnknownDevice,
    LampController,
  ];

  static final $core.Map<$core.int, DeviceType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DeviceType? valueOf($core.int value) => _byValue[value];

  const DeviceType._($core.int v, $core.String n) : super(v, n);
}

class MeasurementType extends $pb.ProtobufEnum {
  static const MeasurementType UnknownMeasurment = MeasurementType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UnknownMeasurment');
  static const MeasurementType Illuminance = MeasurementType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Illuminance');
  static const MeasurementType Voltage = MeasurementType._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Voltage');
  static const MeasurementType Current = MeasurementType._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Current');
  static const MeasurementType Power = MeasurementType._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Power');
  static const MeasurementType Energy = MeasurementType._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Energy');
  static const MeasurementType Frequency = MeasurementType._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Frequency');
  static const MeasurementType PowerFactor = MeasurementType._(7, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'PowerFactor');

  static const $core.List<MeasurementType> values = <MeasurementType> [
    UnknownMeasurment,
    Illuminance,
    Voltage,
    Current,
    Power,
    Energy,
    Frequency,
    PowerFactor,
  ];

  static final $core.Map<$core.int, MeasurementType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static MeasurementType? valueOf($core.int value) => _byValue[value];

  const MeasurementType._($core.int v, $core.String n) : super(v, n);
}

class MeasurementStatus extends $pb.ProtobufEnum {
  static const MeasurementStatus Invalid = MeasurementStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Invalid');
  static const MeasurementStatus Valid = MeasurementStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'Valid');

  static const $core.List<MeasurementStatus> values = <MeasurementStatus> [
    Invalid,
    Valid,
  ];

  static final $core.Map<$core.int, MeasurementStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static MeasurementStatus? valueOf($core.int value) => _byValue[value];

  const MeasurementStatus._($core.int v, $core.String n) : super(v, n);
}

