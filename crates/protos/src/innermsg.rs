// This file is @generated by prost-build.
/// Innermsg
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Innermsg {
    #[prost(enumeration = "Identity", tag = "1")]
    pub identity: i32,
    #[prost(enumeration = "Action", tag = "2")]
    pub action: i32,
    /// for vlc
    #[prost(enumeration = "PushType", tag = "3")]
    pub push_type: i32,
    #[prost(message, optional, tag = "4")]
    pub message: ::core::option::Option<super::zmessage::ZMessage>,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// for verifying or threshold signatures
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Identity {
    Client = 0,
    Server = 1,
    Init = 2,
}
impl Identity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Identity::Client => "IDENTITY_CLIENT",
            Identity::Server => "IDENTITY_SERVER",
            Identity::Init => "IDENTITY_INIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDENTITY_CLIENT" => Some(Self::Client),
            "IDENTITY_SERVER" => Some(Self::Server),
            "IDENTITY_INIT" => Some(Self::Init),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Read = 0,
    Write = 1,
    ReadReply = 2,
    WriteReply = 3,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Read => "ACTION_READ",
            Action::Write => "ACTION_WRITE",
            Action::ReadReply => "ACTION_READ_REPLY",
            Action::WriteReply => "ACTION_WRITE_REPLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_READ" => Some(Self::Read),
            "ACTION_WRITE" => Some(Self::Write),
            "ACTION_READ_REPLY" => Some(Self::ReadReply),
            "ACTION_WRITE_REPLY" => Some(Self::WriteReply),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PushType {
    Direct = 0,
    Broadcast = 1,
}
impl PushType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PushType::Direct => "PUSH_TYPE_DIRECT",
            PushType::Broadcast => "PUSH_TYPE_BROADCAST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PUSH_TYPE_DIRECT" => Some(Self::Direct),
            "PUSH_TYPE_BROADCAST" => Some(Self::Broadcast),
            _ => None,
        }
    }
}
