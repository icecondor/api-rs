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

//! Generated file from `proto/friendship.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_3;

#[derive(PartialEq,Clone,Default,Debug,::serde::Serialize,::serde::Deserialize)]
// @@protoc_insertion_point(message:icecondor.Friendship)
pub struct Friendship {
    // message fields
    // @@protoc_insertion_point(field:icecondor.Friendship.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Friendship.user_id)
    pub user_id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Friendship.friend_user_id)
    pub friend_user_id: ::std::string::String,
    // @@protoc_insertion_point(field:icecondor.Friendship.created_at)
    pub created_at: ::std::string::String,
    // special fields
#[serde(skip)]
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Friendship {
    fn default() -> &'a Friendship {
        <Friendship as ::protobuf::Message>::default_instance()
    }
}

impl Friendship {
    pub fn new() -> Friendship {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Friendship| { &m.id },
            |m: &mut Friendship| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_id",
            |m: &Friendship| { &m.user_id },
            |m: &mut Friendship| { &mut m.user_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "friend_user_id",
            |m: &Friendship| { &m.friend_user_id },
            |m: &mut Friendship| { &mut m.friend_user_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "created_at",
            |m: &Friendship| { &m.created_at },
            |m: &mut Friendship| { &mut m.created_at },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Friendship>(
            "Friendship",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Friendship {
    const NAME: &'static str = "Friendship";

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
                    self.user_id = is.read_string()?;
                },
                26 => {
                    self.friend_user_id = is.read_string()?;
                },
                34 => {
                    self.created_at = is.read_string()?;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.user_id);
        }
        if !self.friend_user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.friend_user_id);
        }
        if !self.created_at.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.created_at);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.user_id.is_empty() {
            os.write_string(2, &self.user_id)?;
        }
        if !self.friend_user_id.is_empty() {
            os.write_string(3, &self.friend_user_id)?;
        }
        if !self.created_at.is_empty() {
            os.write_string(4, &self.created_at)?;
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

    fn new() -> Friendship {
        Friendship::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.user_id.clear();
        self.friend_user_id.clear();
        self.created_at.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Friendship {
        static instance: Friendship = Friendship {
            id: ::std::string::String::new(),
            user_id: ::std::string::String::new(),
            friend_user_id: ::std::string::String::new(),
            created_at: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Friendship {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Friendship").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Friendship {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Friendship {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16proto/friendship.proto\x12\ticecondor\"z\n\nFriendship\x12\x0e\n\
    \x02id\x18\x01\x20\x01(\tR\x02id\x12\x17\n\x07user_id\x18\x02\x20\x01(\t\
    R\x06userId\x12$\n\x0efriend_user_id\x18\x03\x20\x01(\tR\x0cfriendUserId\
    \x12\x1d\n\ncreated_at\x18\x04\x20\x01(\tR\tcreatedAtJ\x90\x02\n\x06\x12\
    \x04\0\0\x08\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\
    \x01\0\x12\n\n\n\x02\x04\0\x12\x04\x03\0\x08\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x03\x08\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\x04\x02\x10\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x04\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x04\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04\x0e\x0f\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x05\x02\x15\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x05\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x05\t\x10\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x05\x13\x14\n\x0b\n\x04\x04\0\x02\x02\x12\x03\
    \x06\x02\x1c\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x06\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x06\t\x17\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x06\x1a\x1b\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x07\x02\x18\n\x0c\
    \n\x05\x04\0\x02\x03\x05\x12\x03\x07\x02\x08\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03\x07\t\x13\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x07\x16\x17\
    b\x06proto3\
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
            messages.push(Friendship::generated_message_descriptor_data());
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
