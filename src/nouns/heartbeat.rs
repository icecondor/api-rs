// This file is generated by rust-protobuf 3.0.3. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `proto/heartbeat.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_3;

#[derive(PartialEq,Clone,Default,Debug,::serde::Serialize,::serde::Deserialize)]
// @@protoc_insertion_point(message:icecondor.Heartbeat)
pub struct Heartbeat {
    // message fields
    // @@protoc_insertion_point(field:icecondor.Heartbeat.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.date)
    pub date: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.received_at)
    pub received_at: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.user_id)
    pub user_id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.device_id)
    pub device_id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.charging)
    pub charging: bool,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.battery_percentage)
    pub battery_percentage: i32,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.cell_data)
    pub cell_data: bool,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.wifi_data)
    pub wifi_data: bool,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.memory_free)
    pub memory_free: u64,
    // @@protoc_insertion_point(field:icecondor.Heartbeat.memory_total)
    pub memory_total: u64,
    // special fields
#[serde(skip)]
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Heartbeat {
    fn default() -> &'a Heartbeat {
        <Heartbeat as ::protobuf::Message>::default_instance()
    }
}

impl Heartbeat {
    pub fn new() -> Heartbeat {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Heartbeat| { &m.id },
            |m: &mut Heartbeat| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "date",
            |m: &Heartbeat| { &m.date },
            |m: &mut Heartbeat| { &mut m.date },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "received_at",
            |m: &Heartbeat| { &m.received_at },
            |m: &mut Heartbeat| { &mut m.received_at },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_id",
            |m: &Heartbeat| { &m.user_id },
            |m: &mut Heartbeat| { &mut m.user_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "device_id",
            |m: &Heartbeat| { &m.device_id },
            |m: &mut Heartbeat| { &mut m.device_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "charging",
            |m: &Heartbeat| { &m.charging },
            |m: &mut Heartbeat| { &mut m.charging },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battery_percentage",
            |m: &Heartbeat| { &m.battery_percentage },
            |m: &mut Heartbeat| { &mut m.battery_percentage },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cell_data",
            |m: &Heartbeat| { &m.cell_data },
            |m: &mut Heartbeat| { &mut m.cell_data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wifi_data",
            |m: &Heartbeat| { &m.wifi_data },
            |m: &mut Heartbeat| { &mut m.wifi_data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "memory_free",
            |m: &Heartbeat| { &m.memory_free },
            |m: &mut Heartbeat| { &mut m.memory_free },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "memory_total",
            |m: &Heartbeat| { &m.memory_total },
            |m: &mut Heartbeat| { &mut m.memory_total },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Heartbeat>(
            "Heartbeat",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Heartbeat {
    const NAME: &'static str = "Heartbeat";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.id = is.read_string()?;
                },
                18 => {
                    self.date = is.read_string()?;
                },
                26 => {
                    self.received_at = is.read_string()?;
                },
                34 => {
                    self.user_id = is.read_string()?;
                },
                42 => {
                    self.device_id = is.read_string()?;
                },
                48 => {
                    self.charging = is.read_bool()?;
                },
                56 => {
                    self.battery_percentage = is.read_int32()?;
                },
                64 => {
                    self.cell_data = is.read_bool()?;
                },
                72 => {
                    self.wifi_data = is.read_bool()?;
                },
                80 => {
                    self.memory_free = is.read_uint64()?;
                },
                88 => {
                    self.memory_total = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.date.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.date);
        }
        if !self.received_at.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.received_at);
        }
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.user_id);
        }
        if !self.device_id.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.device_id);
        }
        if self.charging != false {
            my_size += 1 + 1;
        }
        if self.battery_percentage != 0 {
            my_size += ::protobuf::rt::int32_size(7, self.battery_percentage);
        }
        if self.cell_data != false {
            my_size += 1 + 1;
        }
        if self.wifi_data != false {
            my_size += 1 + 1;
        }
        if self.memory_free != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.memory_free);
        }
        if self.memory_total != 0 {
            my_size += ::protobuf::rt::uint64_size(11, self.memory_total);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.date.is_empty() {
            os.write_string(2, &self.date)?;
        }
        if !self.received_at.is_empty() {
            os.write_string(3, &self.received_at)?;
        }
        if !self.user_id.is_empty() {
            os.write_string(4, &self.user_id)?;
        }
        if !self.device_id.is_empty() {
            os.write_string(5, &self.device_id)?;
        }
        if self.charging != false {
            os.write_bool(6, self.charging)?;
        }
        if self.battery_percentage != 0 {
            os.write_int32(7, self.battery_percentage)?;
        }
        if self.cell_data != false {
            os.write_bool(8, self.cell_data)?;
        }
        if self.wifi_data != false {
            os.write_bool(9, self.wifi_data)?;
        }
        if self.memory_free != 0 {
            os.write_uint64(10, self.memory_free)?;
        }
        if self.memory_total != 0 {
            os.write_uint64(11, self.memory_total)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Heartbeat {
        Heartbeat::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.date.clear();
        self.received_at.clear();
        self.user_id.clear();
        self.device_id.clear();
        self.charging = false;
        self.battery_percentage = 0;
        self.cell_data = false;
        self.wifi_data = false;
        self.memory_free = 0;
        self.memory_total = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Heartbeat {
        static instance: Heartbeat = Heartbeat {
            id: ::std::string::String::new(),
            date: ::std::string::String::new(),
            received_at: ::std::string::String::new(),
            user_id: ::std::string::String::new(),
            device_id: ::std::string::String::new(),
            charging: false,
            battery_percentage: 0,
            cell_data: false,
            wifi_data: false,
            memory_free: 0,
            memory_total: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Heartbeat {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Heartbeat").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Heartbeat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Heartbeat {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15proto/heartbeat.proto\x12\ticecondor\"\xcf\x02\n\tHeartbeat\x12\
    \x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04date\x18\x02\x20\x01\
    (\tR\x04date\x12\x1f\n\x0breceived_at\x18\x03\x20\x01(\tR\nreceivedAt\
    \x12\x17\n\x07user_id\x18\x04\x20\x01(\tR\x06userId\x12\x1b\n\tdevice_id\
    \x18\x05\x20\x01(\tR\x08deviceId\x12\x1a\n\x08charging\x18\x06\x20\x01(\
    \x08R\x08charging\x12-\n\x12battery_percentage\x18\x07\x20\x01(\x05R\x11\
    batteryPercentage\x12\x1b\n\tcell_data\x18\x08\x20\x01(\x08R\x08cellData\
    \x12\x1b\n\twifi_data\x18\t\x20\x01(\x08R\x08wifiData\x12\x1f\n\x0bmemor\
    y_free\x18\n\x20\x01(\x04R\nmemoryFree\x12!\n\x0cmemory_total\x18\x0b\
    \x20\x01(\x04R\x0bmemoryTotalJ\x91\x05\n\x06\x12\x04\0\0\x10\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\x12\n\n\n\x02\x04\
    \0\x12\x04\x03\0\x10\x01\n\n\n\x03\x04\0\x01\x12\x03\x03\x08\x11\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x04\x02\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x04\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\t\x0b\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x04\x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x05\x02\x12\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x05\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x05\t\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x05\x10\x11\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x06\x02\x19\n\x0c\n\
    \x05\x04\0\x02\x02\x05\x12\x03\x06\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\x06\t\x14\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x06\x17\x18\n\
    \x0b\n\x04\x04\0\x02\x03\x12\x03\x07\x02\x15\n\x0c\n\x05\x04\0\x02\x03\
    \x05\x12\x03\x07\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x07\t\x10\
    \n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x07\x13\x14\n\x0b\n\x04\x04\0\x02\
    \x04\x12\x03\x08\x02\x17\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x08\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x08\t\x12\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03\x08\x15\x16\n\x0b\n\x04\x04\0\x02\x05\x12\x03\n\x02\
    \x14\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\n\x02\x06\n\x0c\n\x05\x04\0\
    \x02\x05\x01\x12\x03\n\x07\x0f\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\n\
    \x12\x13\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x0b\x02\x1f\n\x0c\n\x05\x04\0\
    \x02\x06\x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\
    \x0b\x08\x1a\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0b\x1d\x1e\n\x0b\n\
    \x04\x04\0\x02\x07\x12\x03\x0c\x02\x15\n\x0c\n\x05\x04\0\x02\x07\x05\x12\
    \x03\x0c\x02\x06\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x0c\x07\x10\n\x0c\
    \n\x05\x04\0\x02\x07\x03\x12\x03\x0c\x13\x14\n\x0b\n\x04\x04\0\x02\x08\
    \x12\x03\r\x02\x15\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x03\r\x02\x06\n\x0c\
    \n\x05\x04\0\x02\x08\x01\x12\x03\r\x07\x10\n\x0c\n\x05\x04\0\x02\x08\x03\
    \x12\x03\r\x13\x14\n\x0b\n\x04\x04\0\x02\t\x12\x03\x0e\x02\x1a\n\x0c\n\
    \x05\x04\0\x02\t\x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\0\x02\t\x01\x12\
    \x03\x0e\t\x14\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\x0e\x17\x19\n\x0b\n\
    \x04\x04\0\x02\n\x12\x03\x0f\x02\x1b\n\x0c\n\x05\x04\0\x02\n\x05\x12\x03\
    \x0f\x02\x08\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03\x0f\t\x15\n\x0c\n\x05\
    \x04\0\x02\n\x03\x12\x03\x0f\x18\x1ab\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Heartbeat::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}