use finschia_std_derive::CosmwasmExt;
/// LegacyABCIResponses retains the responses
/// of the legacy ABCI calls during block processing.
/// Note ReponseDeliverTx is renamed to ExecTxResult but they are semantically the same
/// Kept for backwards compatibility for versions prior to v0.38
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
#[proto_message(type_url = "/cometbft.state.v1.LegacyABCIResponses")]
pub struct LegacyAbciResponses {
    #[prost(message, repeated, tag = "1")]
    pub deliver_txs: ::prost::alloc::vec::Vec<super::super::abci::v1::ExecTxResult>,
    #[prost(message, optional, tag = "2")]
    pub end_block: ::core::option::Option<ResponseEndBlock>,
    #[prost(message, optional, tag = "3")]
    pub begin_block: ::core::option::Option<ResponseBeginBlock>,
}
/// ResponseBeginBlock is kept for backward compatibility for versions prior to v0.38,
/// as it was then defined in the cometbft.abci packages.
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
#[proto_message(type_url = "/cometbft.state.v1.ResponseBeginBlock")]
pub struct ResponseBeginBlock {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<super::super::abci::v1::Event>,
}
/// ResponseEndBlock is kept for backward compatibility for versions prior to v0.38,
/// its earlier revisions were defined in the cometbft.abci packages.
/// It uses an updated definition for the consensus_param_updates field to keep the
/// generated data types interoperable with the latest protocol.
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
#[proto_message(type_url = "/cometbft.state.v1.ResponseEndBlock")]
pub struct ResponseEndBlock {
    #[prost(message, repeated, tag = "1")]
    pub validator_updates: ::prost::alloc::vec::Vec<super::super::abci::v1::ValidatorUpdate>,
    #[prost(message, optional, tag = "2")]
    pub consensus_param_updates: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<super::super::abci::v1::Event>,
}
/// ValidatorsInfo represents the latest validator set, or the last height it changed
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
#[proto_message(type_url = "/cometbft.state.v1.ValidatorsInfo")]
pub struct ValidatorsInfo {
    #[prost(message, optional, tag = "1")]
    pub validator_set: ::core::option::Option<super::super::types::v1::ValidatorSet>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_height_changed: i64,
}
/// ConsensusParamsInfo represents the latest consensus params, or the last height it changed
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
#[proto_message(type_url = "/cometbft.state.v1.ConsensusParamsInfo")]
pub struct ConsensusParamsInfo {
    #[prost(message, optional, tag = "1")]
    pub consensus_params: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_height_changed: i64,
}
/// ABCIResponsesInfo retains the responses of the ABCI calls during block processing.
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
#[proto_message(type_url = "/cometbft.state.v1.ABCIResponsesInfo")]
pub struct AbciResponsesInfo {
    /// Retains the responses of the legacy ABCI calls during block processing.
    #[prost(message, optional, tag = "1")]
    pub legacy_abci_responses: ::core::option::Option<LegacyAbciResponses>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(message, optional, tag = "3")]
    pub finalize_block: ::core::option::Option<super::super::abci::v1::FinalizeBlockResponse>,
}
/// Version is a message for storing versioning information.
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
#[proto_message(type_url = "/cometbft.state.v1.Version")]
pub struct Version {
    #[prost(message, optional, tag = "1")]
    pub consensus: ::core::option::Option<super::super::version::v1::Consensus>,
    #[prost(string, tag = "2")]
    pub software: ::prost::alloc::string::String,
}
/// State represents the state of the blockchain.
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
#[proto_message(type_url = "/cometbft.state.v1.State")]
pub struct State {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<Version>,
    /// immutable
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "14")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub initial_height: i64,
    /// LastBlockHeight=0 at genesis (ie. block(H=0) does not exist)
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_block_height: i64,
    #[prost(message, optional, tag = "4")]
    #[serde(alias = "last_blockID")]
    pub last_block_id: ::core::option::Option<super::super::types::v1::BlockId>,
    #[prost(message, optional, tag = "5")]
    pub last_block_time: ::core::option::Option<crate::shim::Timestamp>,
    /// LastValidators is used to validate block.LastCommit.
    /// Validators are persisted to the database separately every time they change,
    /// so we can query for historical validator sets.
    /// Note that if s.LastBlockHeight causes a valset change,
    /// we set s.LastHeightValidatorsChanged = s.LastBlockHeight + 1 + 1
    /// Extra +1 due to nextValSet delay.
    #[prost(message, optional, tag = "6")]
    pub next_validators: ::core::option::Option<super::super::types::v1::ValidatorSet>,
    #[prost(message, optional, tag = "7")]
    pub validators: ::core::option::Option<super::super::types::v1::ValidatorSet>,
    #[prost(message, optional, tag = "8")]
    pub last_validators: ::core::option::Option<super::super::types::v1::ValidatorSet>,
    #[prost(int64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_height_validators_changed: i64,
    /// Consensus parameters used for validating blocks.
    /// Changes returned by EndBlock and updated after Commit.
    #[prost(message, optional, tag = "10")]
    pub consensus_params: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    #[prost(int64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_height_consensus_params_changed: i64,
    /// Merkle root of the results from executing prev block
    #[prost(bytes = "vec", tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub last_results_hash: ::prost::alloc::vec::Vec<u8>,
    /// the latest AppHash we've received from calling abci.Commit()
    #[prost(bytes = "vec", tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}