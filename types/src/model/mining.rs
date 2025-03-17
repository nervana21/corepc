// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Mining ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use std::collections::BTreeMap;

use bitcoin::{block, BlockHash, CompactTarget, SignedAmount, Transaction, Txid, Weight, Wtxid};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `getblocktemplate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplate {
    /// The preferred block version.
    pub version: block::Version,
    /// Specific block rules that are to be enforced.
    pub rules: Vec<String>,
    /// Set of pending, supported versionbit (BIP 9) softfork deployments.
    ///
    /// Map of rules name to bit number - identifies the bit number as indicating acceptance and
    /// readiness for the named softfork rule.
    pub version_bits_available: BTreeMap<String, u32>,
    /// Bit mask of versionbits the server requires set in submissions.
    pub version_bits_required: u32,
    /// The hash of current highest block.
    pub previous_block_hash: BlockHash,
    /// Contents of non-coinbase transactions that should be included in the next block.
    pub transactions: Vec<BlockTemplateTransaction>,
    /// Data that should be included in the coinbase's scriptSig content.
    ///
    /// Key name is to be ignored, and value included in scriptSig.
    pub coinbase_aux: BTreeMap<String, String>,
    /// Maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis).
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub coinbase_value: SignedAmount,
    /// The hash target.
    pub target: Vec<u8>,
    /// The minimum timestamp appropriate for next block time in seconds since epoch (Jan 1 1970 GMT).
    pub min_time: u32,
    /// List of ways the block template may be changed.
    ///
    /// A way the block template may be changed, e.g. 'time', 'transactions', 'prevblock'
    pub mutable: Vec<String>,
    /// A range of valid nonces.
    pub nonce_range: String,
    /// Limit of sigops in blocks.
    pub sigop_limit: u32,
    /// Limit of block size.
    pub size_limit: u32,
    /// Limit of block weight.
    pub weight_limit: u32,
    /// Current timestamp in seconds since epoch (Jan 1 1970 GMT).
    pub current_time: u64,
    /// Compressed target of next block.
    pub bits: CompactTarget,
    /// The height of the next block,
    pub height: u32,
}

/// Contents of non-coinbase transactions that should be included in the next block.
///
/// Returned as part of `getblocktemplate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BlockTemplateTransaction {
    /// The transaction.
    pub data: Transaction,
    /// The transaction ID.
    pub txid: Txid,
    /// The segwit transaction ID.
    pub wtxid: Wtxid,
    /// Array of numbers.
    ///
    /// Transactions before this one (by 1-based index in 'transactions' list) that must be present in the final block if this one is.
    pub depends: Vec<u32>,
    /// Difference in value between transaction inputs and outputs (in satoshis); for coinbase
    /// transactions, this is a negative Number of the total collected block fees (ie, not including
    /// the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there
    /// isn't one.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub fee: SignedAmount,
    /// Total SigOps cost, as counted for purposes of block limits; if key is not present, sigop
    /// cost is unknown and clients MUST NOT assume it is zero.
    pub sigops: u32,
    /// Total transaction weight, as counted for purposes of block limits.
    pub weight: Weight,
}

/// Models the result of JSON-RPC method `getprioritisedtransactions`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetPrioritisedTransactions(pub BTreeMap<Txid, PrioritisedTransaction>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PrioritisedTransaction {
    /// Transaction fee delta in satoshis.
    pub fee_delta: i64,
    /// Whether this transaction is currently in mempool.
    pub in_mempool: bool,
}
