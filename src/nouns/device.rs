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

//! Generated file from `proto/device.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_3;

#[derive(PartialEq,Clone,Default,Debug,::serde::Serialize,::serde::Deserialize)]
// @@protoc_insertion_point(message:icecondor.Device)
pub struct Device {
    // message fields
    // @@protoc_insertion_point(field:icecondor.Device.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.created_at)
    pub created_at: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.user_id)
    pub user_id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.device_id)
    pub device_id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.make)
    pub make: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.model)
    pub model: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.vdop)
    pub vdop: f32,
    // @@protoc_insertion_point(field:icecondor.Device.hdop)
    pub hdop: f32,
    // @@protoc_insertion_point(field:icecondor.Device.battery_life)
    pub battery_life: f32,
    // @@protoc_insertion_point(field:icecondor.Device.os_name)
    pub os_name: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.os_version)
    pub os_version: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.app_name)
    pub app_name: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.app_version)
    pub app_version: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Device.archived)
    pub archived: bool,
    // special fields
#[serde(skip)]
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Device {
    fn default() -> &'a Device {
        <Device as ::protobuf::Message>::default_instance()
    }
}

impl Device {
    pub fn new() -> Device {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(15);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Device| { &m.id },
            |m: &mut Device| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "created_at",
            |m: &Device| { &m.created_at },
            |m: &mut Device| { &mut m.created_at },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_id",
            |m: &Device| { &m.user_id },
            |m: &mut Device| { &mut m.user_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "device_id",
            |m: &Device| { &m.device_id },
            |m: &mut Device| { &mut m.device_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Device| { &m.name },
            |m: &mut Device| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "make",
            |m: &Device| { &m.make },
            |m: &mut Device| { &mut m.make },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "model",
            |m: &Device| { &m.model },
            |m: &mut Device| { &mut m.model },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "vdop",
            |m: &Device| { &m.vdop },
            |m: &mut Device| { &mut m.vdop },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hdop",
            |m: &Device| { &m.hdop },
            |m: &mut Device| { &mut m.hdop },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battery_life",
            |m: &Device| { &m.battery_life },
            |m: &mut Device| { &mut m.battery_life },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "os_name",
            |m: &Device| { &m.os_name },
            |m: &mut Device| { &mut m.os_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "os_version",
            |m: &Device| { &m.os_version },
            |m: &mut Device| { &mut m.os_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "app_name",
            |m: &Device| { &m.app_name },
            |m: &mut Device| { &mut m.app_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "app_version",
            |m: &Device| { &m.app_version },
            |m: &mut Device| { &mut m.app_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "archived",
            |m: &Device| { &m.archived },
            |m: &mut Device| { &mut m.archived },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Device>(
            "Device",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Device {
    const NAME: &'static str = "Device";

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
                    self.created_at = is.read_string()?;
                },
                26 => {
                    self.user_id = is.read_string()?;
                },
                34 => {
                    self.device_id = is.read_string()?;
                },
                42 => {
                    self.name = is.read_string()?;
                },
                50 => {
                    self.make = is.read_string()?;
                },
                58 => {
                    self.model = is.read_string()?;
                },
                69 => {
                    self.vdop = is.read_float()?;
                },
                77 => {
                    self.hdop = is.read_float()?;
                },
                85 => {
                    self.battery_life = is.read_float()?;
                },
                90 => {
                    self.os_name = is.read_string()?;
                },
                98 => {
                    self.os_version = is.read_string()?;
                },
                106 => {
                    self.app_name = is.read_string()?;
                },
                114 => {
                    self.app_version = is.read_string()?;
                },
                120 => {
                    self.archived = is.read_bool()?;
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
        if !self.created_at.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.created_at);
        }
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.user_id);
        }
        if !self.device_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.device_id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.name);
        }
        if !self.make.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.make);
        }
        if !self.model.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.model);
        }
        if self.vdop != 0. {
            my_size += 1 + 4;
        }
        if self.hdop != 0. {
            my_size += 1 + 4;
        }
        if self.battery_life != 0. {
            my_size += 1 + 4;
        }
        if !self.os_name.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.os_name);
        }
        if !self.os_version.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.os_version);
        }
        if !self.app_name.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.app_name);
        }
        if !self.app_version.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.app_version);
        }
        if self.archived != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.created_at.is_empty() {
            os.write_string(2, &self.created_at)?;
        }
        if !self.user_id.is_empty() {
            os.write_string(3, &self.user_id)?;
        }
        if !self.device_id.is_empty() {
            os.write_string(4, &self.device_id)?;
        }
        if !self.name.is_empty() {
            os.write_string(5, &self.name)?;
        }
        if !self.make.is_empty() {
            os.write_string(6, &self.make)?;
        }
        if !self.model.is_empty() {
            os.write_string(7, &self.model)?;
        }
        if self.vdop != 0. {
            os.write_float(8, self.vdop)?;
        }
        if self.hdop != 0. {
            os.write_float(9, self.hdop)?;
        }
        if self.battery_life != 0. {
            os.write_float(10, self.battery_life)?;
        }
        if !self.os_name.is_empty() {
            os.write_string(11, &self.os_name)?;
        }
        if !self.os_version.is_empty() {
            os.write_string(12, &self.os_version)?;
        }
        if !self.app_name.is_empty() {
            os.write_string(13, &self.app_name)?;
        }
        if !self.app_version.is_empty() {
            os.write_string(14, &self.app_version)?;
        }
        if self.archived != false {
            os.write_bool(15, self.archived)?;
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

    fn new() -> Device {
        Device::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.created_at.clear();
        self.user_id.clear();
        self.device_id.clear();
        self.name.clear();
        self.make.clear();
        self.model.clear();
        self.vdop = 0.;
        self.hdop = 0.;
        self.battery_life = 0.;
        self.os_name.clear();
        self.os_version.clear();
        self.app_name.clear();
        self.app_version.clear();
        self.archived = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Device {
        static instance: Device = Device {
            id: ::std::string::String::new(),
            created_at: ::std::string::String::new(),
            user_id: ::std::string::String::new(),
            device_id: ::std::string::String::new(),
            name: ::std::string::String::new(),
            make: ::std::string::String::new(),
            model: ::std::string::String::new(),
            vdop: 0.,
            hdop: 0.,
            battery_life: 0.,
            os_name: ::std::string::String::new(),
            os_version: ::std::string::String::new(),
            app_name: ::std::string::String::new(),
            app_version: ::std::string::String::new(),
            archived: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Device {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Device").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Device {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Device {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12proto/device.proto\x12\ticecondor\"\x86\x03\n\x06Device\x12\x0e\n\
    \x02id\x18\x01\x20\x01(\tR\x02id\x12\x1d\n\ncreated_at\x18\x02\x20\x01(\
    \tR\tcreatedAt\x12\x17\n\x07user_id\x18\x03\x20\x01(\tR\x06userId\x12\
    \x1b\n\tdevice_id\x18\x04\x20\x01(\tR\x08deviceId\x12\x12\n\x04name\x18\
    \x05\x20\x01(\tR\x04name\x12\x12\n\x04make\x18\x06\x20\x01(\tR\x04make\
    \x12\x14\n\x05model\x18\x07\x20\x01(\tR\x05model\x12\x12\n\x04vdop\x18\
    \x08\x20\x01(\x02R\x04vdop\x12\x12\n\x04hdop\x18\t\x20\x01(\x02R\x04hdop\
    \x12!\n\x0cbattery_life\x18\n\x20\x01(\x02R\x0bbatteryLife\x12\x17\n\x07\
    os_name\x18\x0b\x20\x01(\tR\x06osName\x12\x1d\n\nos_version\x18\x0c\x20\
    \x01(\tR\tosVersion\x12\x19\n\x08app_name\x18\r\x20\x01(\tR\x07appName\
    \x12\x1f\n\x0bapp_version\x18\x0e\x20\x01(\tR\nappVersion\x12\x1a\n\x08a\
    rchived\x18\x0f\x20\x01(\x08R\x08archivedJ\xed\x06\n\x06\x12\x04\0\0\x13\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\x12\n\
    \n\n\x02\x04\0\x12\x04\x03\0\x13\x01\n\n\n\x03\x04\0\x01\x12\x03\x03\x08\
    \x0e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x04\x02\x10\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x04\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\t\x0b\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04\x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x05\x02\x18\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x05\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x05\t\x13\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03\x05\x16\x17\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x06\x02\x15\n\
    \x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x06\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x06\t\x10\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x06\x13\
    \x14\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x07\x02\x17\n\x0c\n\x05\x04\0\x02\
    \x03\x05\x12\x03\x07\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x07\t\
    \x12\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x07\x15\x16\n\x0b\n\x04\x04\0\
    \x02\x04\x12\x03\x08\x02\x12\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x08\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x08\t\r\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03\x08\x10\x11\n\x0b\n\x04\x04\0\x02\x05\x12\x03\t\x02\
    \x12\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x05\x01\x12\x03\t\t\r\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\t\x10\
    \x11\n\x0b\n\x04\x04\0\x02\x06\x12\x03\n\x02\x13\n\x0c\n\x05\x04\0\x02\
    \x06\x05\x12\x03\n\x02\x08\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\n\t\x0e\
    \n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\n\x11\x12\n\x0b\n\x04\x04\0\x02\
    \x07\x12\x03\x0b\x02\x11\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x0b\x02\
    \x07\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x0b\x08\x0c\n\x0c\n\x05\x04\0\
    \x02\x07\x03\x12\x03\x0b\x0f\x10\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x0c\
    \x02\x11\n\x0c\n\x05\x04\0\x02\x08\x05\x12\x03\x0c\x02\x07\n\x0c\n\x05\
    \x04\0\x02\x08\x01\x12\x03\x0c\x08\x0c\n\x0c\n\x05\x04\0\x02\x08\x03\x12\
    \x03\x0c\x0f\x10\n\x0b\n\x04\x04\0\x02\t\x12\x03\r\x02\x1a\n\x0c\n\x05\
    \x04\0\x02\t\x05\x12\x03\r\x02\x07\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03\r\
    \x08\x14\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\r\x17\x19\n\x0b\n\x04\x04\0\
    \x02\n\x12\x03\x0e\x02\x16\n\x0c\n\x05\x04\0\x02\n\x05\x12\x03\x0e\x02\
    \x08\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03\x0e\t\x10\n\x0c\n\x05\x04\0\x02\
    \n\x03\x12\x03\x0e\x13\x15\n\x0b\n\x04\x04\0\x02\x0b\x12\x03\x0f\x02\x19\
    \n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x0b\x01\x12\x03\x0f\t\x13\n\x0c\n\x05\x04\0\x02\x0b\x03\x12\x03\x0f\x16\
    \x18\n\x0b\n\x04\x04\0\x02\x0c\x12\x03\x10\x02\x17\n\x0c\n\x05\x04\0\x02\
    \x0c\x05\x12\x03\x10\x02\x08\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03\x10\t\
    \x11\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03\x10\x14\x16\n\x0b\n\x04\x04\0\
    \x02\r\x12\x03\x11\x02\x1a\n\x0c\n\x05\x04\0\x02\r\x05\x12\x03\x11\x02\
    \x08\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03\x11\t\x14\n\x0c\n\x05\x04\0\x02\
    \r\x03\x12\x03\x11\x17\x19\n\x0b\n\x04\x04\0\x02\x0e\x12\x03\x12\x02\x15\
    \n\x0c\n\x05\x04\0\x02\x0e\x05\x12\x03\x12\x02\x06\n\x0c\n\x05\x04\0\x02\
    \x0e\x01\x12\x03\x12\x07\x0f\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03\x12\
    \x12\x14b\x06proto3\
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
            messages.push(Device::generated_message_descriptor_data());
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
