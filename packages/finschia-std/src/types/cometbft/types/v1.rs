use finschia_std_derive::CosmwasmExt;
/// ConsensusParams contains consensus critical parameters that determine the
/// validity of blocks.
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
#[proto_message(type_url = "/cometbft.types.v1.ConsensusParams")]
pub struct ConsensusParams {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockParams>,
    #[prost(message, optional, tag = "2")]
    pub evidence: ::core::option::Option<EvidenceParams>,
    #[prost(message, optional, tag = "3")]
    pub validator: ::core::option::Option<ValidatorParams>,
    #[prost(message, optional, tag = "4")]
    pub version: ::core::option::Option<VersionParams>,
    /// Use FeatureParams.vote_extensions_enable_height instead
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub abci: ::core::option::Option<AbciParams>,
    #[prost(message, optional, tag = "6")]
    pub synchrony: ::core::option::Option<SynchronyParams>,
    #[prost(message, optional, tag = "7")]
    pub feature: ::core::option::Option<FeatureParams>,
}
/// BlockParams define limits on the block size and gas.
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
#[proto_message(type_url = "/cometbft.types.v1.BlockParams")]
pub struct BlockParams {
    /// Maximum size of a block, in bytes.
    ///
    /// Must be greater or equal to -1 and cannot be greater than the hard-coded
    /// maximum block size, which is 100MB.
    ///
    /// If set to -1, the limit is the hard-coded maximum block size.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_bytes: i64,
    /// Maximum gas wanted by transactions included in a block.
    ///
    /// Must be greater or equal to -1. If set to -1, no limit is enforced.
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_gas: i64,
}
/// EvidenceParams determine the validity of evidences of Byzantine behavior.
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
#[proto_message(type_url = "/cometbft.types.v1.EvidenceParams")]
pub struct EvidenceParams {
    /// Maximum age of evidence, in blocks.
    ///
    /// The recommended formula for calculating it is max_age_duration / {average
    /// block time}.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_age_num_blocks: i64,
    /// Maximum age of evidence, in time.
    ///
    /// The recommended value of is should correspond to the application's
    /// "unbonding period" or other similar mechanism for handling
    /// Nothing-At-Stake attacks.
    /// See: <https://github.com/ethereum/wiki/wiki/Proof-of-Stake-FAQ#what-is-the-nothing-at-stake-problem-and-how-can-it-be-fixed.>
    #[prost(message, optional, tag = "2")]
    pub max_age_duration: ::core::option::Option<crate::shim::Duration>,
    /// Maximum size in bytes of evidence allowed to be included in a block.
    ///
    /// It should fall comfortably under the maximum size of a block.
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_bytes: i64,
}
/// ValidatorParams restrict the public key types validators can use.
///
/// NOTE: uses ABCI public keys naming, not Amino names.
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
#[proto_message(type_url = "/cometbft.types.v1.ValidatorParams")]
pub struct ValidatorParams {
    #[prost(string, repeated, tag = "1")]
    pub pub_key_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// VersionParams contain the version of specific components of CometBFT.
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
#[proto_message(type_url = "/cometbft.types.v1.VersionParams")]
pub struct VersionParams {
    /// The ABCI application version.
    ///
    /// It was named app_version in CometBFT 0.34.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub app: u64,
}
/// HashedParams is a subset of ConsensusParams.
///
/// It is hashed into the Header.ConsensusHash.
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
#[proto_message(type_url = "/cometbft.types.v1.HashedParams")]
pub struct HashedParams {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_max_bytes: i64,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_max_gas: i64,
}
/// SynchronyParams determine the validity of block timestamps.
///
/// These parameters are part of the Proposer-Based Timestamps (PBTS) algorithm.
/// For more information on the relationship of the synchrony parameters to
/// block timestamps validity, refer to the PBTS specification:
/// <https://github.com/tendermint/spec/blob/master/spec/consensus/proposer-based-timestamp/README.md>
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
#[proto_message(type_url = "/cometbft.types.v1.SynchronyParams")]
pub struct SynchronyParams {
    /// Bound for how skewed a proposer's clock may be from any validator on the
    /// network while still producing valid proposals.
    #[prost(message, optional, tag = "1")]
    pub precision: ::core::option::Option<crate::shim::Duration>,
    /// Bound for how long a proposal message may take to reach all validators on
    /// a network and still be considered valid.
    #[prost(message, optional, tag = "2")]
    pub message_delay: ::core::option::Option<crate::shim::Duration>,
}
/// FeatureParams configure the height from which features of CometBFT are enabled.
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
#[proto_message(type_url = "/cometbft.types.v1.FeatureParams")]
pub struct FeatureParams {
    /// First height during which vote extensions will be enabled.
    ///
    /// During the specified height, and for all subsequent heights, precommit
    /// messages that do not contain valid extension data will be considered
    /// invalid. Prior to this height, or when this height is set to 0, vote
    /// extensions will not be used or accepted by validators on the network.
    ///
    /// Once enabled, vote extensions will be created by the application in
    /// ExtendVote, validated by the application in VerifyVoteExtension, and
    /// used by the application in PrepareProposal, when proposing the next block.
    ///
    /// Cannot be set to heights lower or equal to the current blockchain height.
    #[prost(message, optional, tag = "1")]
    pub vote_extensions_enable_height: ::core::option::Option<i64>,
    /// Height at which Proposer-Based Timestamps (PBTS) will be enabled.
    ///
    /// From the specified height, and for all subsequent heights, the PBTS
    /// algorithm will be used to produce and validate block timestamps. Prior to
    /// this height, or when this height is set to 0, the legacy BFT Time
    /// algorithm is used to produce and validate timestamps.
    ///
    /// Cannot be set to heights lower or equal to the current blockchain height.
    #[prost(message, optional, tag = "2")]
    pub pbts_enable_height: ::core::option::Option<i64>,
}
/// ABCIParams is deprecated and its contents moved to FeatureParams
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
#[proto_message(type_url = "/cometbft.types.v1.ABCIParams")]
#[deprecated]
pub struct AbciParams {
    /// vote_extensions_enable_height has been deprecated.
    /// Instead, use FeatureParams.vote_extensions_enable_height.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub vote_extensions_enable_height: i64,
}
/// ValidatorSet defines a set of validators.
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
#[proto_message(type_url = "/cometbft.types.v1.ValidatorSet")]
pub struct ValidatorSet {
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(message, optional, tag = "2")]
    pub proposer: ::core::option::Option<Validator>,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_voting_power: i64,
}
/// Validator represents a node participating in the consensus protocol.
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
#[proto_message(type_url = "/cometbft.types.v1.Validator")]
pub struct Validator {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::core::option::Option<super::super::crypto::v1::PublicKey>,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub voting_power: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposer_priority: i64,
}
/// SimpleValidator is a Validator, which is serialized and hashed in consensus.
/// Address is removed because it's redundant with the pubkey.
/// Proposer priority is removed because it changes every round.
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
#[proto_message(type_url = "/cometbft.types.v1.SimpleValidator")]
pub struct SimpleValidator {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<super::super::crypto::v1::PublicKey>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub voting_power: i64,
}
/// BlockIdFlag indicates which BlockID the signature is for
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum BlockIdFlag {
    /// Indicates an error condition
    Unknown = 0,
    /// The vote was not received
    Absent = 1,
    /// Voted for the block that received the majority
    Commit = 2,
    /// Voted for nil
    Nil = 3,
}
impl BlockIdFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockIdFlag::Unknown => "BLOCK_ID_FLAG_UNKNOWN",
            BlockIdFlag::Absent => "BLOCK_ID_FLAG_ABSENT",
            BlockIdFlag::Commit => "BLOCK_ID_FLAG_COMMIT",
            BlockIdFlag::Nil => "BLOCK_ID_FLAG_NIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_ID_FLAG_UNKNOWN" => Some(Self::Unknown),
            "BLOCK_ID_FLAG_ABSENT" => Some(Self::Absent),
            "BLOCK_ID_FLAG_COMMIT" => Some(Self::Commit),
            "BLOCK_ID_FLAG_NIL" => Some(Self::Nil),
            _ => None,
        }
    }
}
/// Header of the parts set for a block.
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
#[proto_message(type_url = "/cometbft.types.v1.PartSetHeader")]
pub struct PartSetHeader {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total: u32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// Part of the block.
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
#[proto_message(type_url = "/cometbft.types.v1.Part")]
pub struct Part {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<super::super::crypto::v1::Proof>,
}
/// BlockID defines the unique ID of a block as its hash and its `PartSetHeader`.
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
#[proto_message(type_url = "/cometbft.types.v1.BlockID")]
pub struct BlockId {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub part_set_header: ::core::option::Option<PartSetHeader>,
}
/// Header defines the structure of a block header.
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
#[proto_message(type_url = "/cometbft.types.v1.Header")]
pub struct Header {
    /// basic block info
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::super::version::v1::Consensus>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag = "5")]
    #[serde(alias = "last_blockID")]
    pub last_block_id: ::core::option::Option<BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub last_commit_hash: ::prost::alloc::vec::Vec<u8>,
    /// transactions
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes = "vec", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes = "vec", tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub consensus_hash: ::prost::alloc::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes = "vec", tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// root hash of all results from the txs from the previous block
    #[prost(bytes = "vec", tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub last_results_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes = "vec", tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// original proposer of the block
    #[prost(bytes = "vec", tag = "14")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// Data contains the set of transactions included in the block
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
#[proto_message(type_url = "/cometbft.types.v1.Data")]
pub struct Data {
    /// Txs that will be applied by state @ block.Height+1.
    /// NOTE: not all txs here are valid.  We're just agreeing on the order first.
    /// This means that block.AppHash does not include these txs.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Vote represents a prevote or precommit vote from validators for
/// consensus.
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
#[proto_message(type_url = "/cometbft.types.v1.Vote")]
pub struct Vote {
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub r#type: i32,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(int32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i32,
    /// zero if vote is nil.
    #[prost(message, optional, tag = "4")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_index: i32,
    /// Vote signature by the validator if they participated in consensus for the
    /// associated block.
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension provided by the application. Only valid for precommit
    /// messages.
    #[prost(bytes = "vec", tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension signature by the validator if they participated in
    /// consensus for the associated block.
    /// Only valid for precommit messages.
    #[prost(bytes = "vec", tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub extension_signature: ::prost::alloc::vec::Vec<u8>,
}
/// Commit contains the evidence that a block was committed by a set of validators.
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
#[proto_message(type_url = "/cometbft.types.v1.Commit")]
pub struct Commit {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(int32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i32,
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, repeated, tag = "4")]
    pub signatures: ::prost::alloc::vec::Vec<CommitSig>,
}
/// CommitSig is a part of the Vote included in a Commit.
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
#[proto_message(type_url = "/cometbft.types.v1.CommitSig")]
pub struct CommitSig {
    #[prost(enumeration = "BlockIdFlag", tag = "1")]
    #[serde(alias = "blockID_flag")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_id_flag: i32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// ExtendedCommit is a Commit with ExtendedCommitSig.
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
#[proto_message(type_url = "/cometbft.types.v1.ExtendedCommit")]
pub struct ExtendedCommit {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(int32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i32,
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, repeated, tag = "4")]
    pub extended_signatures: ::prost::alloc::vec::Vec<ExtendedCommitSig>,
}
/// ExtendedCommitSig retains all the same fields as CommitSig but adds vote
/// extension-related fields. We use two signatures to ensure backwards compatibility.
/// That is the digest of the original signature is still the same in prior versions
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
#[proto_message(type_url = "/cometbft.types.v1.ExtendedCommitSig")]
pub struct ExtendedCommitSig {
    #[prost(enumeration = "BlockIdFlag", tag = "1")]
    #[serde(alias = "blockID_flag")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_id_flag: i32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension data
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension signature
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub extension_signature: ::prost::alloc::vec::Vec<u8>,
}
/// Block proposal.
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
#[proto_message(type_url = "/cometbft.types.v1.Proposal")]
pub struct Proposal {
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub r#type: i32,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(int32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i32,
    #[prost(int32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pol_round: i32,
    #[prost(message, optional, tag = "5")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// SignedHeader contains a Header(H) and Commit(H+1) with signatures of validators who signed it.
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
#[proto_message(type_url = "/cometbft.types.v1.SignedHeader")]
pub struct SignedHeader {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub commit: ::core::option::Option<Commit>,
}
/// LightBlock is a combination of SignedHeader and ValidatorSet. It is used by light clients.
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
#[proto_message(type_url = "/cometbft.types.v1.LightBlock")]
pub struct LightBlock {
    #[prost(message, optional, tag = "1")]
    pub signed_header: ::core::option::Option<SignedHeader>,
    #[prost(message, optional, tag = "2")]
    pub validator_set: ::core::option::Option<ValidatorSet>,
}
/// BlockMeta contains meta information about a block.
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
#[proto_message(type_url = "/cometbft.types.v1.BlockMeta")]
pub struct BlockMeta {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_size: i64,
    #[prost(message, optional, tag = "3")]
    pub header: ::core::option::Option<Header>,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub num_txs: i64,
}
/// TxProof represents a Merkle proof of the presence of a transaction in the Merkle tree.
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
#[proto_message(type_url = "/cometbft.types.v1.TxProof")]
pub struct TxProof {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<super::super::crypto::v1::Proof>,
}
/// SignedMsgType is a type of signed message in the consensus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum SignedMsgType {
    /// Unknown
    Unknown = 0,
    /// Prevote
    Prevote = 1,
    /// Precommit
    Precommit = 2,
    /// Proposal
    Proposal = 32,
}
impl SignedMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignedMsgType::Unknown => "SIGNED_MSG_TYPE_UNKNOWN",
            SignedMsgType::Prevote => "SIGNED_MSG_TYPE_PREVOTE",
            SignedMsgType::Precommit => "SIGNED_MSG_TYPE_PRECOMMIT",
            SignedMsgType::Proposal => "SIGNED_MSG_TYPE_PROPOSAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNED_MSG_TYPE_UNKNOWN" => Some(Self::Unknown),
            "SIGNED_MSG_TYPE_PREVOTE" => Some(Self::Prevote),
            "SIGNED_MSG_TYPE_PRECOMMIT" => Some(Self::Precommit),
            "SIGNED_MSG_TYPE_PROPOSAL" => Some(Self::Proposal),
            _ => None,
        }
    }
}
/// Evidence is a generic type for wrapping evidence of misbehavior by a validator.
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
#[proto_message(type_url = "/cometbft.types.v1.Evidence")]
pub struct Evidence {
    /// The type of evidence.
    #[prost(oneof = "evidence::Sum", tags = "1, 2")]
    pub sum: ::core::option::Option<evidence::Sum>,
}
/// Nested message and enum types in `Evidence`.
pub mod evidence {
    use finschia_std_derive::CosmwasmExt;
    /// The type of evidence.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Sum {
        #[prost(message, tag = "1")]
        DuplicateVoteEvidence(super::DuplicateVoteEvidence),
        #[prost(message, tag = "2")]
        LightClientAttackEvidence(super::LightClientAttackEvidence),
    }
}
/// DuplicateVoteEvidence contains evidence of a validator signed two conflicting votes.
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
#[proto_message(type_url = "/cometbft.types.v1.DuplicateVoteEvidence")]
pub struct DuplicateVoteEvidence {
    #[prost(message, optional, tag = "1")]
    pub vote_a: ::core::option::Option<Vote>,
    #[prost(message, optional, tag = "2")]
    pub vote_b: ::core::option::Option<Vote>,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_voting_power: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_power: i64,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
/// LightClientAttackEvidence contains evidence of a set of validators attempting to mislead a light client.
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
#[proto_message(type_url = "/cometbft.types.v1.LightClientAttackEvidence")]
pub struct LightClientAttackEvidence {
    #[prost(message, optional, tag = "1")]
    pub conflicting_block: ::core::option::Option<LightBlock>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub common_height: i64,
    #[prost(message, repeated, tag = "3")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_voting_power: i64,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
/// EvidenceList is a list of evidence.
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
#[proto_message(type_url = "/cometbft.types.v1.EvidenceList")]
pub struct EvidenceList {
    #[prost(message, repeated, tag = "1")]
    pub evidence: ::prost::alloc::vec::Vec<Evidence>,
}
/// Block defines the structure of a block in the CometBFT blockchain.
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
#[proto_message(type_url = "/cometbft.types.v1.Block")]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    #[prost(message, optional, tag = "3")]
    pub evidence: ::core::option::Option<EvidenceList>,
    #[prost(message, optional, tag = "4")]
    pub last_commit: ::core::option::Option<Commit>,
}
/// EventDataRoundState is emitted with each new round step.
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
#[proto_message(type_url = "/cometbft.types.v1.EventDataRoundState")]
pub struct EventDataRoundState {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(int32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i32,
    #[prost(string, tag = "3")]
    pub step: ::prost::alloc::string::String,
}
/// CanonicalBlockID is a canonical representation of a BlockID, which gets
/// serialized and signed.
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
#[proto_message(type_url = "/cometbft.types.v1.CanonicalBlockID")]
pub struct CanonicalBlockId {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub part_set_header: ::core::option::Option<CanonicalPartSetHeader>,
}
/// CanonicalPartSetHeader is a canonical representation of a PartSetHeader,
/// which gets serialized and signed.
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
#[proto_message(type_url = "/cometbft.types.v1.CanonicalPartSetHeader")]
pub struct CanonicalPartSetHeader {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total: u32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// CanonicalProposal is a canonical representation of a Proposal, which gets
/// serialized and signed.
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
#[proto_message(type_url = "/cometbft.types.v1.CanonicalProposal")]
pub struct CanonicalProposal {
    /// type alias for byte
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pol_round: i64,
    #[prost(message, optional, tag = "5")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "7")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// CanonicalVote is a canonical representation of a Vote, which gets
/// serialized and signed.
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
#[proto_message(type_url = "/cometbft.types.v1.CanonicalVote")]
pub struct CanonicalVote {
    /// type alias for byte
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i64,
    #[prost(message, optional, tag = "4")]
    #[serde(alias = "blockID")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "6")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// CanonicalVoteExtension provides us a way to serialize a vote extension from
/// a particular validator such that we can sign over those serialized bytes.
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
#[proto_message(type_url = "/cometbft.types.v1.CanonicalVoteExtension")]
pub struct CanonicalVoteExtension {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    #[prost(sfixed64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(sfixed64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i64,
    #[prost(string, tag = "4")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
