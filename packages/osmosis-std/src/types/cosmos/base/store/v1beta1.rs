use osmosis_std_derive::CosmwasmExt;
/// CommitInfo defines commit information used by the multi-store when committing
/// a version/height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.store.v1beta1.CommitInfo")]
pub struct CommitInfo {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: i64,
    #[prost(message, repeated, tag = "2")]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
}
/// StoreInfo defines store-specific commit information. It contains a reference
/// between a store name and the commit ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.store.v1beta1.StoreInfo")]
pub struct StoreInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "commitID")]
    pub commit_id: ::core::option::Option<CommitId>,
}
/// CommitID defines the committment information when a specific store is
/// committed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.store.v1beta1.CommitID")]
pub struct CommitId {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: i64,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// StoreKVPair is a KVStore KVPair used for listening to state changes (Sets and Deletes)
/// It optionally includes the StoreKey for the originating KVStore and a Boolean flag to distinguish between Sets and
/// Deletes
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.store.v1beta1.StoreKVPair")]
pub struct StoreKvPair {
    /// the store key for the KVStore this pair originates from
    #[prost(string, tag = "1")]
    pub store_key: ::prost::alloc::string::String,
    /// true indicates a delete operation, false indicates a set operation
    #[prost(bool, tag = "2")]
    pub delete: bool,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
