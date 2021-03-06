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

//! Generated file from `proto/user.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_3;

#[derive(PartialEq,Clone,Default,Debug,::serde::Serialize,::serde::Deserialize)]
// @@protoc_insertion_point(message:icecondor.User)
pub struct User {
    // message fields
    // @@protoc_insertion_point(field:icecondor.User.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.User.created_at)
    pub created_at: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.User.email)
    pub email: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.User.username)
    pub username: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.User.time_zone)
    pub time_zone: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.User.paid)
    pub paid: ::std::string::String,
    // special fields
#[serde(skip)]
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a User {
    fn default() -> &'a User {
        <User as ::protobuf::Message>::default_instance()
    }
}

impl User {
    pub fn new() -> User {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &User| { &m.id },
            |m: &mut User| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "created_at",
            |m: &User| { &m.created_at },
            |m: &mut User| { &mut m.created_at },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "email",
            |m: &User| { &m.email },
            |m: &mut User| { &mut m.email },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "username",
            |m: &User| { &m.username },
            |m: &mut User| { &mut m.username },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_zone",
            |m: &User| { &m.time_zone },
            |m: &mut User| { &mut m.time_zone },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "paid",
            |m: &User| { &m.paid },
            |m: &mut User| { &mut m.paid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<User>(
            "User",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for User {
    const NAME: &'static str = "User";

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
                    self.email = is.read_string()?;
                },
                34 => {
                    self.username = is.read_string()?;
                },
                42 => {
                    self.time_zone = is.read_string()?;
                },
                50 => {
                    self.paid = is.read_string()?;
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
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.email);
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.username);
        }
        if !self.time_zone.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.time_zone);
        }
        if !self.paid.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.paid);
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
        if !self.email.is_empty() {
            os.write_string(3, &self.email)?;
        }
        if !self.username.is_empty() {
            os.write_string(4, &self.username)?;
        }
        if !self.time_zone.is_empty() {
            os.write_string(5, &self.time_zone)?;
        }
        if !self.paid.is_empty() {
            os.write_string(6, &self.paid)?;
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

    fn new() -> User {
        User::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.created_at.clear();
        self.email.clear();
        self.username.clear();
        self.time_zone.clear();
        self.paid.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static User {
        static instance: User = User {
            id: ::std::string::String::new(),
            created_at: ::std::string::String::new(),
            email: ::std::string::String::new(),
            username: ::std::string::String::new(),
            time_zone: ::std::string::String::new(),
            paid: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for User {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("User").unwrap()).clone()
    }
}

impl ::std::fmt::Display for User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for User {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10proto/user.proto\x12\ticecondor\"\x98\x01\n\x04User\x12\x0e\n\x02i\
    d\x18\x01\x20\x01(\tR\x02id\x12\x1d\n\ncreated_at\x18\x02\x20\x01(\tR\tc\
    reatedAt\x12\x14\n\x05email\x18\x03\x20\x01(\tR\x05email\x12\x1a\n\x08us\
    ername\x18\x04\x20\x01(\tR\x08username\x12\x1b\n\ttime_zone\x18\x05\x20\
    \x01(\tR\x08timeZone\x12\x12\n\x04paid\x18\x06\x20\x01(\tR\x04paidJ\xfe\
    \x02\n\x06\x12\x04\0\0\n\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\
    \x02\x12\x03\x01\0\x12\n\n\n\x02\x04\0\x12\x04\x03\0\n\x01\n\n\n\x03\x04\
    \0\x01\x12\x03\x03\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\x04\x02\x10\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x04\x02\x08\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x04\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04\x0e\x0f\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x05\x02\x18\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x05\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x05\t\x13\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x05\x16\x17\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\x06\x02\x13\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x06\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x06\t\x0e\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03\x06\x11\x12\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x07\
    \x02\x16\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x07\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03\x07\t\x11\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03\x07\x14\x15\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x08\x02\x17\n\x0c\n\
    \x05\x04\0\x02\x04\x05\x12\x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\
    \x12\x03\x08\t\x12\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x08\x15\x16\n\
    \x0b\n\x04\x04\0\x02\x05\x12\x03\t\x02\x12\n\x0c\n\x05\x04\0\x02\x05\x05\
    \x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\t\t\r\n\x0c\n\
    \x05\x04\0\x02\x05\x03\x12\x03\t\x10\x11b\x06proto3\
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
            messages.push(User::generated_message_descriptor_data());
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
