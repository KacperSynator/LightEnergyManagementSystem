///
//  Generated code. Do not modify.
//  source: light_energy_management_system.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'light_energy_management_system.pbenum.dart';

export 'light_energy_management_system.pbenum.dart';

class Device extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Device', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'light_energy_management_system'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mac')
    ..e<DeviceType>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: DeviceType.UnknownDevice, valueOf: DeviceType.valueOf, enumValues: DeviceType.values)
    ..hasRequiredFields = false
  ;

  Device._() : super();
  factory Device({
    $core.String? name,
    $core.String? mac,
    DeviceType? type,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (mac != null) {
      _result.mac = mac;
    }
    if (type != null) {
      _result.type = type;
    }
    return _result;
  }
  factory Device.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Device.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Device clone() => Device()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Device copyWith(void Function(Device) updates) => super.copyWith((message) => updates(message as Device)) as Device; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Device create() => Device._();
  Device createEmptyInstance() => create();
  static $pb.PbList<Device> createRepeated() => $pb.PbList<Device>();
  @$core.pragma('dart2js:noInline')
  static Device getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Device>(create);
  static Device? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get mac => $_getSZ(1);
  @$pb.TagNumber(2)
  set mac($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMac() => $_has(1);
  @$pb.TagNumber(2)
  void clearMac() => clearField(2);

  @$pb.TagNumber(3)
  DeviceType get type => $_getN(2);
  @$pb.TagNumber(3)
  set type(DeviceType v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasType() => $_has(2);
  @$pb.TagNumber(3)
  void clearType() => clearField(3);
}

class Measurement extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Measurement', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'light_energy_management_system'), createEmptyInstance: create)
    ..a<$core.double>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'value', $pb.PbFieldType.OF)
    ..e<MeasurementType>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: MeasurementType.UnknownMeasurment, valueOf: MeasurementType.valueOf, enumValues: MeasurementType.values)
    ..e<MeasurementStatus>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status', $pb.PbFieldType.OE, defaultOrMaker: MeasurementStatus.Invalid, valueOf: MeasurementStatus.valueOf, enumValues: MeasurementStatus.values)
    ..hasRequiredFields = false
  ;

  Measurement._() : super();
  factory Measurement({
    $core.double? value,
    MeasurementType? type,
    MeasurementStatus? status,
  }) {
    final _result = create();
    if (value != null) {
      _result.value = value;
    }
    if (type != null) {
      _result.type = type;
    }
    if (status != null) {
      _result.status = status;
    }
    return _result;
  }
  factory Measurement.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Measurement.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Measurement clone() => Measurement()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Measurement copyWith(void Function(Measurement) updates) => super.copyWith((message) => updates(message as Measurement)) as Measurement; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Measurement create() => Measurement._();
  Measurement createEmptyInstance() => create();
  static $pb.PbList<Measurement> createRepeated() => $pb.PbList<Measurement>();
  @$core.pragma('dart2js:noInline')
  static Measurement getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Measurement>(create);
  static Measurement? _defaultInstance;

  @$pb.TagNumber(1)
  $core.double get value => $_getN(0);
  @$pb.TagNumber(1)
  set value($core.double v) { $_setFloat(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasValue() => $_has(0);
  @$pb.TagNumber(1)
  void clearValue() => clearField(1);

  @$pb.TagNumber(2)
  MeasurementType get type => $_getN(1);
  @$pb.TagNumber(2)
  set type(MeasurementType v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasType() => $_has(1);
  @$pb.TagNumber(2)
  void clearType() => clearField(2);

  @$pb.TagNumber(3)
  MeasurementStatus get status => $_getN(2);
  @$pb.TagNumber(3)
  set status(MeasurementStatus v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasStatus() => $_has(2);
  @$pb.TagNumber(3)
  void clearStatus() => clearField(3);
}

class DeviceMeasurments extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DeviceMeasurments', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'light_energy_management_system'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..pc<Measurement>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'measurements', $pb.PbFieldType.PM, subBuilder: Measurement.create)
    ..hasRequiredFields = false
  ;

  DeviceMeasurments._() : super();
  factory DeviceMeasurments({
    $fixnum.Int64? timestamp,
    $core.Iterable<Measurement>? measurements,
  }) {
    final _result = create();
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (measurements != null) {
      _result.measurements.addAll(measurements);
    }
    return _result;
  }
  factory DeviceMeasurments.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DeviceMeasurments.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DeviceMeasurments clone() => DeviceMeasurments()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DeviceMeasurments copyWith(void Function(DeviceMeasurments) updates) => super.copyWith((message) => updates(message as DeviceMeasurments)) as DeviceMeasurments; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeviceMeasurments create() => DeviceMeasurments._();
  DeviceMeasurments createEmptyInstance() => create();
  static $pb.PbList<DeviceMeasurments> createRepeated() => $pb.PbList<DeviceMeasurments>();
  @$core.pragma('dart2js:noInline')
  static DeviceMeasurments getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DeviceMeasurments>(create);
  static DeviceMeasurments? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get timestamp => $_getI64(0);
  @$pb.TagNumber(1)
  set timestamp($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<Measurement> get measurements => $_getList(1);
}

class DataPacket extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'DataPacket', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'light_energy_management_system'), createEmptyInstance: create)
    ..aOM<Device>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'device', subBuilder: Device.create)
    ..pc<DeviceMeasurments>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'deviceMeasurements', $pb.PbFieldType.PM, subBuilder: DeviceMeasurments.create)
    ..hasRequiredFields = false
  ;

  DataPacket._() : super();
  factory DataPacket({
    Device? device,
    $core.Iterable<DeviceMeasurments>? deviceMeasurements,
  }) {
    final _result = create();
    if (device != null) {
      _result.device = device;
    }
    if (deviceMeasurements != null) {
      _result.deviceMeasurements.addAll(deviceMeasurements);
    }
    return _result;
  }
  factory DataPacket.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory DataPacket.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  DataPacket clone() => DataPacket()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  DataPacket copyWith(void Function(DataPacket) updates) => super.copyWith((message) => updates(message as DataPacket)) as DataPacket; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DataPacket create() => DataPacket._();
  DataPacket createEmptyInstance() => create();
  static $pb.PbList<DataPacket> createRepeated() => $pb.PbList<DataPacket>();
  @$core.pragma('dart2js:noInline')
  static DataPacket getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<DataPacket>(create);
  static DataPacket? _defaultInstance;

  @$pb.TagNumber(1)
  Device get device => $_getN(0);
  @$pb.TagNumber(1)
  set device(Device v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasDevice() => $_has(0);
  @$pb.TagNumber(1)
  void clearDevice() => clearField(1);
  @$pb.TagNumber(1)
  Device ensureDevice() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<DeviceMeasurments> get deviceMeasurements => $_getList(1);
}

