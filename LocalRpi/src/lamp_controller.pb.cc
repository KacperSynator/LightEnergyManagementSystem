// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: lamp_controller.proto

#include "lamp_controller.pb.h"

#include <algorithm>

#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/extension_set.h>
#include <google/protobuf/wire_format_lite.h>
#include <google/protobuf/descriptor.h>
#include <google/protobuf/generated_message_reflection.h>
#include <google/protobuf/reflection_ops.h>
#include <google/protobuf/wire_format.h>
// @@protoc_insertion_point(includes)
#include <google/protobuf/port_def.inc>

PROTOBUF_PRAGMA_INIT_SEG

namespace _pb = ::PROTOBUF_NAMESPACE_ID;
namespace _pbi = _pb::internal;

namespace lamp_controller {
PROTOBUF_CONSTEXPR LampData::LampData(
    ::_pbi::ConstantInitialized): _impl_{
    /*decltype(_impl_.name_)*/{&::_pbi::fixed_address_empty_string, ::_pbi::ConstantInitialized{}}
  , /*decltype(_impl_.illuminance_)*/0
  , /*decltype(_impl_.voltage_)*/0
  , /*decltype(_impl_.current_)*/0
  , /*decltype(_impl_.power_)*/0
  , /*decltype(_impl_.energy_)*/0
  , /*decltype(_impl_.frequency_)*/0
  , /*decltype(_impl_.power_factor_)*/0
  , /*decltype(_impl_._cached_size_)*/{}} {}
struct LampDataDefaultTypeInternal {
  PROTOBUF_CONSTEXPR LampDataDefaultTypeInternal()
      : _instance(::_pbi::ConstantInitialized{}) {}
  ~LampDataDefaultTypeInternal() {}
  union {
    LampData _instance;
  };
};
PROTOBUF_ATTRIBUTE_NO_DESTROY PROTOBUF_CONSTINIT PROTOBUF_ATTRIBUTE_INIT_PRIORITY1 LampDataDefaultTypeInternal _LampData_default_instance_;
}  // namespace lamp_controller
static ::_pb::Metadata file_level_metadata_lamp_5fcontroller_2eproto[1];
static constexpr ::_pb::EnumDescriptor const** file_level_enum_descriptors_lamp_5fcontroller_2eproto = nullptr;
static constexpr ::_pb::ServiceDescriptor const** file_level_service_descriptors_lamp_5fcontroller_2eproto = nullptr;

const uint32_t TableStruct_lamp_5fcontroller_2eproto::offsets[] PROTOBUF_SECTION_VARIABLE(protodesc_cold) = {
  ~0u,  // no _has_bits_
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _internal_metadata_),
  ~0u,  // no _extensions_
  ~0u,  // no _oneof_case_
  ~0u,  // no _weak_field_map_
  ~0u,  // no _inlined_string_donated_
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.name_),
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.illuminance_),
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.voltage_),
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.current_),
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.power_),
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.energy_),
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.frequency_),
  PROTOBUF_FIELD_OFFSET(::lamp_controller::LampData, _impl_.power_factor_),
};
static const ::_pbi::MigrationSchema schemas[] PROTOBUF_SECTION_VARIABLE(protodesc_cold) = {
  { 0, -1, -1, sizeof(::lamp_controller::LampData)},
};

static const ::_pb::Message* const file_default_instances[] = {
  &::lamp_controller::_LampData_default_instance_._instance,
};

const char descriptor_table_protodef_lamp_5fcontroller_2eproto[] PROTOBUF_SECTION_VARIABLE(protodesc_cold) =
  "\n\025lamp_controller.proto\022\017lamp_controller"
  "\"\227\001\n\010LampData\022\014\n\004name\030\001 \001(\t\022\023\n\013illuminan"
  "ce\030\002 \001(\002\022\017\n\007voltage\030\003 \001(\002\022\017\n\007current\030\004 \001"
  "(\002\022\r\n\005power\030\005 \001(\002\022\016\n\006energy\030\006 \001(\002\022\021\n\tfre"
  "quency\030\007 \001(\002\022\024\n\014power_factor\030\010 \001(\002b\006prot"
  "o3"
  ;
static ::_pbi::once_flag descriptor_table_lamp_5fcontroller_2eproto_once;
const ::_pbi::DescriptorTable descriptor_table_lamp_5fcontroller_2eproto = {
    false, false, 202, descriptor_table_protodef_lamp_5fcontroller_2eproto,
    "lamp_controller.proto",
    &descriptor_table_lamp_5fcontroller_2eproto_once, nullptr, 0, 1,
    schemas, file_default_instances, TableStruct_lamp_5fcontroller_2eproto::offsets,
    file_level_metadata_lamp_5fcontroller_2eproto, file_level_enum_descriptors_lamp_5fcontroller_2eproto,
    file_level_service_descriptors_lamp_5fcontroller_2eproto,
};
PROTOBUF_ATTRIBUTE_WEAK const ::_pbi::DescriptorTable* descriptor_table_lamp_5fcontroller_2eproto_getter() {
  return &descriptor_table_lamp_5fcontroller_2eproto;
}

// Force running AddDescriptors() at dynamic initialization time.
PROTOBUF_ATTRIBUTE_INIT_PRIORITY2 static ::_pbi::AddDescriptorsRunner dynamic_init_dummy_lamp_5fcontroller_2eproto(&descriptor_table_lamp_5fcontroller_2eproto);
namespace lamp_controller {

// ===================================================================

class LampData::_Internal {
 public:
};

LampData::LampData(::PROTOBUF_NAMESPACE_ID::Arena* arena,
                         bool is_message_owned)
  : ::PROTOBUF_NAMESPACE_ID::Message(arena, is_message_owned) {
  SharedCtor(arena, is_message_owned);
  // @@protoc_insertion_point(arena_constructor:lamp_controller.LampData)
}
LampData::LampData(const LampData& from)
  : ::PROTOBUF_NAMESPACE_ID::Message() {
  LampData* const _this = this; (void)_this;
  new (&_impl_) Impl_{
      decltype(_impl_.name_){}
    , decltype(_impl_.illuminance_){}
    , decltype(_impl_.voltage_){}
    , decltype(_impl_.current_){}
    , decltype(_impl_.power_){}
    , decltype(_impl_.energy_){}
    , decltype(_impl_.frequency_){}
    , decltype(_impl_.power_factor_){}
    , /*decltype(_impl_._cached_size_)*/{}};

  _internal_metadata_.MergeFrom<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(from._internal_metadata_);
  _impl_.name_.InitDefault();
  #ifdef PROTOBUF_FORCE_COPY_DEFAULT_STRING
    _impl_.name_.Set("", GetArenaForAllocation());
  #endif // PROTOBUF_FORCE_COPY_DEFAULT_STRING
  if (!from._internal_name().empty()) {
    _this->_impl_.name_.Set(from._internal_name(), 
      _this->GetArenaForAllocation());
  }
  ::memcpy(&_impl_.illuminance_, &from._impl_.illuminance_,
    static_cast<size_t>(reinterpret_cast<char*>(&_impl_.power_factor_) -
    reinterpret_cast<char*>(&_impl_.illuminance_)) + sizeof(_impl_.power_factor_));
  // @@protoc_insertion_point(copy_constructor:lamp_controller.LampData)
}

inline void LampData::SharedCtor(
    ::_pb::Arena* arena, bool is_message_owned) {
  (void)arena;
  (void)is_message_owned;
  new (&_impl_) Impl_{
      decltype(_impl_.name_){}
    , decltype(_impl_.illuminance_){0}
    , decltype(_impl_.voltage_){0}
    , decltype(_impl_.current_){0}
    , decltype(_impl_.power_){0}
    , decltype(_impl_.energy_){0}
    , decltype(_impl_.frequency_){0}
    , decltype(_impl_.power_factor_){0}
    , /*decltype(_impl_._cached_size_)*/{}
  };
  _impl_.name_.InitDefault();
  #ifdef PROTOBUF_FORCE_COPY_DEFAULT_STRING
    _impl_.name_.Set("", GetArenaForAllocation());
  #endif // PROTOBUF_FORCE_COPY_DEFAULT_STRING
}

LampData::~LampData() {
  // @@protoc_insertion_point(destructor:lamp_controller.LampData)
  if (auto *arena = _internal_metadata_.DeleteReturnArena<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>()) {
  (void)arena;
    return;
  }
  SharedDtor();
}

inline void LampData::SharedDtor() {
  GOOGLE_DCHECK(GetArenaForAllocation() == nullptr);
  _impl_.name_.Destroy();
}

void LampData::SetCachedSize(int size) const {
  _impl_._cached_size_.Set(size);
}

void LampData::Clear() {
// @@protoc_insertion_point(message_clear_start:lamp_controller.LampData)
  uint32_t cached_has_bits = 0;
  // Prevent compiler warnings about cached_has_bits being unused
  (void) cached_has_bits;

  _impl_.name_.ClearToEmpty();
  ::memset(&_impl_.illuminance_, 0, static_cast<size_t>(
      reinterpret_cast<char*>(&_impl_.power_factor_) -
      reinterpret_cast<char*>(&_impl_.illuminance_)) + sizeof(_impl_.power_factor_));
  _internal_metadata_.Clear<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>();
}

const char* LampData::_InternalParse(const char* ptr, ::_pbi::ParseContext* ctx) {
#define CHK_(x) if (PROTOBUF_PREDICT_FALSE(!(x))) goto failure
  while (!ctx->Done(&ptr)) {
    uint32_t tag;
    ptr = ::_pbi::ReadTag(ptr, &tag);
    switch (tag >> 3) {
      // string name = 1;
      case 1:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 10)) {
          auto str = _internal_mutable_name();
          ptr = ::_pbi::InlineGreedyStringParser(str, ptr, ctx);
          CHK_(ptr);
          CHK_(::_pbi::VerifyUTF8(str, "lamp_controller.LampData.name"));
        } else
          goto handle_unusual;
        continue;
      // float illuminance = 2;
      case 2:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 21)) {
          _impl_.illuminance_ = ::PROTOBUF_NAMESPACE_ID::internal::UnalignedLoad<float>(ptr);
          ptr += sizeof(float);
        } else
          goto handle_unusual;
        continue;
      // float voltage = 3;
      case 3:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 29)) {
          _impl_.voltage_ = ::PROTOBUF_NAMESPACE_ID::internal::UnalignedLoad<float>(ptr);
          ptr += sizeof(float);
        } else
          goto handle_unusual;
        continue;
      // float current = 4;
      case 4:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 37)) {
          _impl_.current_ = ::PROTOBUF_NAMESPACE_ID::internal::UnalignedLoad<float>(ptr);
          ptr += sizeof(float);
        } else
          goto handle_unusual;
        continue;
      // float power = 5;
      case 5:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 45)) {
          _impl_.power_ = ::PROTOBUF_NAMESPACE_ID::internal::UnalignedLoad<float>(ptr);
          ptr += sizeof(float);
        } else
          goto handle_unusual;
        continue;
      // float energy = 6;
      case 6:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 53)) {
          _impl_.energy_ = ::PROTOBUF_NAMESPACE_ID::internal::UnalignedLoad<float>(ptr);
          ptr += sizeof(float);
        } else
          goto handle_unusual;
        continue;
      // float frequency = 7;
      case 7:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 61)) {
          _impl_.frequency_ = ::PROTOBUF_NAMESPACE_ID::internal::UnalignedLoad<float>(ptr);
          ptr += sizeof(float);
        } else
          goto handle_unusual;
        continue;
      // float power_factor = 8;
      case 8:
        if (PROTOBUF_PREDICT_TRUE(static_cast<uint8_t>(tag) == 69)) {
          _impl_.power_factor_ = ::PROTOBUF_NAMESPACE_ID::internal::UnalignedLoad<float>(ptr);
          ptr += sizeof(float);
        } else
          goto handle_unusual;
        continue;
      default:
        goto handle_unusual;
    }  // switch
  handle_unusual:
    if ((tag == 0) || ((tag & 7) == 4)) {
      CHK_(ptr);
      ctx->SetLastTag(tag);
      goto message_done;
    }
    ptr = UnknownFieldParse(
        tag,
        _internal_metadata_.mutable_unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(),
        ptr, ctx);
    CHK_(ptr != nullptr);
  }  // while
message_done:
  return ptr;
failure:
  ptr = nullptr;
  goto message_done;
#undef CHK_
}

uint8_t* LampData::_InternalSerialize(
    uint8_t* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const {
  // @@protoc_insertion_point(serialize_to_array_start:lamp_controller.LampData)
  uint32_t cached_has_bits = 0;
  (void) cached_has_bits;

  // string name = 1;
  if (!this->_internal_name().empty()) {
    ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::VerifyUtf8String(
      this->_internal_name().data(), static_cast<int>(this->_internal_name().length()),
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::SERIALIZE,
      "lamp_controller.LampData.name");
    target = stream->WriteStringMaybeAliased(
        1, this->_internal_name(), target);
  }

  // float illuminance = 2;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_illuminance = this->_internal_illuminance();
  uint32_t raw_illuminance;
  memcpy(&raw_illuminance, &tmp_illuminance, sizeof(tmp_illuminance));
  if (raw_illuminance != 0) {
    target = stream->EnsureSpace(target);
    target = ::_pbi::WireFormatLite::WriteFloatToArray(2, this->_internal_illuminance(), target);
  }

  // float voltage = 3;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_voltage = this->_internal_voltage();
  uint32_t raw_voltage;
  memcpy(&raw_voltage, &tmp_voltage, sizeof(tmp_voltage));
  if (raw_voltage != 0) {
    target = stream->EnsureSpace(target);
    target = ::_pbi::WireFormatLite::WriteFloatToArray(3, this->_internal_voltage(), target);
  }

  // float current = 4;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_current = this->_internal_current();
  uint32_t raw_current;
  memcpy(&raw_current, &tmp_current, sizeof(tmp_current));
  if (raw_current != 0) {
    target = stream->EnsureSpace(target);
    target = ::_pbi::WireFormatLite::WriteFloatToArray(4, this->_internal_current(), target);
  }

  // float power = 5;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_power = this->_internal_power();
  uint32_t raw_power;
  memcpy(&raw_power, &tmp_power, sizeof(tmp_power));
  if (raw_power != 0) {
    target = stream->EnsureSpace(target);
    target = ::_pbi::WireFormatLite::WriteFloatToArray(5, this->_internal_power(), target);
  }

  // float energy = 6;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_energy = this->_internal_energy();
  uint32_t raw_energy;
  memcpy(&raw_energy, &tmp_energy, sizeof(tmp_energy));
  if (raw_energy != 0) {
    target = stream->EnsureSpace(target);
    target = ::_pbi::WireFormatLite::WriteFloatToArray(6, this->_internal_energy(), target);
  }

  // float frequency = 7;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_frequency = this->_internal_frequency();
  uint32_t raw_frequency;
  memcpy(&raw_frequency, &tmp_frequency, sizeof(tmp_frequency));
  if (raw_frequency != 0) {
    target = stream->EnsureSpace(target);
    target = ::_pbi::WireFormatLite::WriteFloatToArray(7, this->_internal_frequency(), target);
  }

  // float power_factor = 8;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_power_factor = this->_internal_power_factor();
  uint32_t raw_power_factor;
  memcpy(&raw_power_factor, &tmp_power_factor, sizeof(tmp_power_factor));
  if (raw_power_factor != 0) {
    target = stream->EnsureSpace(target);
    target = ::_pbi::WireFormatLite::WriteFloatToArray(8, this->_internal_power_factor(), target);
  }

  if (PROTOBUF_PREDICT_FALSE(_internal_metadata_.have_unknown_fields())) {
    target = ::_pbi::WireFormat::InternalSerializeUnknownFieldsToArray(
        _internal_metadata_.unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(::PROTOBUF_NAMESPACE_ID::UnknownFieldSet::default_instance), target, stream);
  }
  // @@protoc_insertion_point(serialize_to_array_end:lamp_controller.LampData)
  return target;
}

size_t LampData::ByteSizeLong() const {
// @@protoc_insertion_point(message_byte_size_start:lamp_controller.LampData)
  size_t total_size = 0;

  uint32_t cached_has_bits = 0;
  // Prevent compiler warnings about cached_has_bits being unused
  (void) cached_has_bits;

  // string name = 1;
  if (!this->_internal_name().empty()) {
    total_size += 1 +
      ::PROTOBUF_NAMESPACE_ID::internal::WireFormatLite::StringSize(
        this->_internal_name());
  }

  // float illuminance = 2;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_illuminance = this->_internal_illuminance();
  uint32_t raw_illuminance;
  memcpy(&raw_illuminance, &tmp_illuminance, sizeof(tmp_illuminance));
  if (raw_illuminance != 0) {
    total_size += 1 + 4;
  }

  // float voltage = 3;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_voltage = this->_internal_voltage();
  uint32_t raw_voltage;
  memcpy(&raw_voltage, &tmp_voltage, sizeof(tmp_voltage));
  if (raw_voltage != 0) {
    total_size += 1 + 4;
  }

  // float current = 4;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_current = this->_internal_current();
  uint32_t raw_current;
  memcpy(&raw_current, &tmp_current, sizeof(tmp_current));
  if (raw_current != 0) {
    total_size += 1 + 4;
  }

  // float power = 5;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_power = this->_internal_power();
  uint32_t raw_power;
  memcpy(&raw_power, &tmp_power, sizeof(tmp_power));
  if (raw_power != 0) {
    total_size += 1 + 4;
  }

  // float energy = 6;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_energy = this->_internal_energy();
  uint32_t raw_energy;
  memcpy(&raw_energy, &tmp_energy, sizeof(tmp_energy));
  if (raw_energy != 0) {
    total_size += 1 + 4;
  }

  // float frequency = 7;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_frequency = this->_internal_frequency();
  uint32_t raw_frequency;
  memcpy(&raw_frequency, &tmp_frequency, sizeof(tmp_frequency));
  if (raw_frequency != 0) {
    total_size += 1 + 4;
  }

  // float power_factor = 8;
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_power_factor = this->_internal_power_factor();
  uint32_t raw_power_factor;
  memcpy(&raw_power_factor, &tmp_power_factor, sizeof(tmp_power_factor));
  if (raw_power_factor != 0) {
    total_size += 1 + 4;
  }

  return MaybeComputeUnknownFieldsSize(total_size, &_impl_._cached_size_);
}

const ::PROTOBUF_NAMESPACE_ID::Message::ClassData LampData::_class_data_ = {
    ::PROTOBUF_NAMESPACE_ID::Message::CopyWithSourceCheck,
    LampData::MergeImpl
};
const ::PROTOBUF_NAMESPACE_ID::Message::ClassData*LampData::GetClassData() const { return &_class_data_; }


void LampData::MergeImpl(::PROTOBUF_NAMESPACE_ID::Message& to_msg, const ::PROTOBUF_NAMESPACE_ID::Message& from_msg) {
  auto* const _this = static_cast<LampData*>(&to_msg);
  auto& from = static_cast<const LampData&>(from_msg);
  // @@protoc_insertion_point(class_specific_merge_from_start:lamp_controller.LampData)
  GOOGLE_DCHECK_NE(&from, _this);
  uint32_t cached_has_bits = 0;
  (void) cached_has_bits;

  if (!from._internal_name().empty()) {
    _this->_internal_set_name(from._internal_name());
  }
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_illuminance = from._internal_illuminance();
  uint32_t raw_illuminance;
  memcpy(&raw_illuminance, &tmp_illuminance, sizeof(tmp_illuminance));
  if (raw_illuminance != 0) {
    _this->_internal_set_illuminance(from._internal_illuminance());
  }
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_voltage = from._internal_voltage();
  uint32_t raw_voltage;
  memcpy(&raw_voltage, &tmp_voltage, sizeof(tmp_voltage));
  if (raw_voltage != 0) {
    _this->_internal_set_voltage(from._internal_voltage());
  }
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_current = from._internal_current();
  uint32_t raw_current;
  memcpy(&raw_current, &tmp_current, sizeof(tmp_current));
  if (raw_current != 0) {
    _this->_internal_set_current(from._internal_current());
  }
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_power = from._internal_power();
  uint32_t raw_power;
  memcpy(&raw_power, &tmp_power, sizeof(tmp_power));
  if (raw_power != 0) {
    _this->_internal_set_power(from._internal_power());
  }
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_energy = from._internal_energy();
  uint32_t raw_energy;
  memcpy(&raw_energy, &tmp_energy, sizeof(tmp_energy));
  if (raw_energy != 0) {
    _this->_internal_set_energy(from._internal_energy());
  }
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_frequency = from._internal_frequency();
  uint32_t raw_frequency;
  memcpy(&raw_frequency, &tmp_frequency, sizeof(tmp_frequency));
  if (raw_frequency != 0) {
    _this->_internal_set_frequency(from._internal_frequency());
  }
  static_assert(sizeof(uint32_t) == sizeof(float), "Code assumes uint32_t and float are the same size.");
  float tmp_power_factor = from._internal_power_factor();
  uint32_t raw_power_factor;
  memcpy(&raw_power_factor, &tmp_power_factor, sizeof(tmp_power_factor));
  if (raw_power_factor != 0) {
    _this->_internal_set_power_factor(from._internal_power_factor());
  }
  _this->_internal_metadata_.MergeFrom<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(from._internal_metadata_);
}

void LampData::CopyFrom(const LampData& from) {
// @@protoc_insertion_point(class_specific_copy_from_start:lamp_controller.LampData)
  if (&from == this) return;
  Clear();
  MergeFrom(from);
}

bool LampData::IsInitialized() const {
  return true;
}

void LampData::InternalSwap(LampData* other) {
  using std::swap;
  auto* lhs_arena = GetArenaForAllocation();
  auto* rhs_arena = other->GetArenaForAllocation();
  _internal_metadata_.InternalSwap(&other->_internal_metadata_);
  ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::InternalSwap(
      &_impl_.name_, lhs_arena,
      &other->_impl_.name_, rhs_arena
  );
  ::PROTOBUF_NAMESPACE_ID::internal::memswap<
      PROTOBUF_FIELD_OFFSET(LampData, _impl_.power_factor_)
      + sizeof(LampData::_impl_.power_factor_)
      - PROTOBUF_FIELD_OFFSET(LampData, _impl_.illuminance_)>(
          reinterpret_cast<char*>(&_impl_.illuminance_),
          reinterpret_cast<char*>(&other->_impl_.illuminance_));
}

::PROTOBUF_NAMESPACE_ID::Metadata LampData::GetMetadata() const {
  return ::_pbi::AssignDescriptors(
      &descriptor_table_lamp_5fcontroller_2eproto_getter, &descriptor_table_lamp_5fcontroller_2eproto_once,
      file_level_metadata_lamp_5fcontroller_2eproto[0]);
}

// @@protoc_insertion_point(namespace_scope)
}  // namespace lamp_controller
PROTOBUF_NAMESPACE_OPEN
template<> PROTOBUF_NOINLINE ::lamp_controller::LampData*
Arena::CreateMaybeMessage< ::lamp_controller::LampData >(Arena* arena) {
  return Arena::CreateMessageInternal< ::lamp_controller::LampData >(arena);
}
PROTOBUF_NAMESPACE_CLOSE

// @@protoc_insertion_point(global_scope)
#include <google/protobuf/port_undef.inc>
