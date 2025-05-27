// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.17`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

pub mod blockchain;
pub mod control;
pub mod generating;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod util;
pub mod wallet;

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{sign_message, Amount, Block, BlockHash, PublicKey, Txid};
use serde::{Deserialize, Serialize};

use crate::client_sync::into_json;
use crate::types::v17::*;

crate::define_jsonrpc_minreq_client!("v17");
crate::impl_client_check_expected_server_version!({ [170200] });

// == Blockchain ==
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getblockcount!();
crate::impl_client_v17__getblockhash!();
crate::impl_client_v17__getblockheader!();
crate::impl_client_v17__getblockstats!();
crate::impl_client_v17__getchaintips!();
crate::impl_client_v17__getchaintxstats!();
crate::impl_client_v17__getdifficulty!();
crate::impl_client_v17__getmempoolancestors!();
crate::impl_client_v17__getmempooldescendants!();
crate::impl_client_v17__getmempoolentry!();
crate::impl_client_v17__getmempoolinfo!();
crate::impl_client_v17__getrawmempool!();
crate::impl_client_v17__gettxout!();
crate::impl_client_v17__gettxoutproof!();
crate::impl_client_v17__gettxoutsetinfo!();
crate::impl_client_v17__preciousblock!();
crate::impl_client_v17__pruneblockchain!();
crate::impl_client_v17__savemempool!();
crate::impl_client_v17__verifychain!();
crate::impl_client_v17__verifytxoutproof!();

// == Control ==
crate::impl_client_v17__getmemoryinfo!();
crate::impl_client_v17__help!();
crate::impl_client_v17__logging!();
crate::impl_client_v17__stop!();
crate::impl_client_v17__uptime!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();
crate::impl_client_v17__generate!();
crate::impl_client_v17__invalidateblock!();

// == Mining ==
crate::impl_client_v17__getblocktemplate!();
crate::impl_client_v17__getmininginfo!();
crate::impl_client_v17__getnetworkhashps!();
crate::impl_client_v17__prioritisetransaction!();
crate::impl_client_v17__submitblock!();

// == Network ==
crate::impl_client_v17__addnode!();
crate::impl_client_v17__clearbanned!();
crate::impl_client_v17__getaddednodeinfo!();
crate::impl_client_v17__getnettotals!();
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_v17__getpeerinfo!();
crate::impl_client_v17__setban!();

// == Rawtransactions ==
crate::impl_client_v17__combinepsbt!();
crate::impl_client_v17__combinerawtransaction!();
crate::impl_client_v17__converttopsbt!();
crate::impl_client_v17__createpsbt!();
crate::impl_client_v17__createrawtransaction!();
crate::impl_client_v17__decodepsbt!();
crate::impl_client_v17__decoderawtransaction!();
crate::impl_client_v17__decodescript!();
crate::impl_client_v17__finalizepsbt!();
crate::impl_client_v17__fundrawtransaction!();
crate::impl_client_v17__getrawtransaction!();
crate::impl_client_v17__sendrawtransaction!();
crate::impl_client_v17__signrawtransaction!();
crate::impl_client_v17__signrawtransactionwithkey!();
crate::impl_client_v17__testmempoolaccept!();

// == Util ==
crate::impl_client_v17__createmultisig!();
crate::impl_client_v17__estimatesmartfee!();
crate::impl_client_v17__signmessagewithprivkey!();
crate::impl_client_v17__validateaddress!();
crate::impl_client_v17__verifymessage!();

// == Wallet ==
crate::impl_client_v17__addmultisigaddress!();
crate::impl_client_v17__bumpfee!();
crate::impl_client_v17__createwallet!();
crate::impl_client_v17__dumpprivkey!();
crate::impl_client_v17__dumpwallet!();
crate::impl_client_v17__getaddressesbylabel!();
crate::impl_client_v17__getaddressinfo!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__getrawchangeaddress!();
crate::impl_client_v17__getreceivedbyaddress!();
crate::impl_client_v17__gettransaction!();
crate::impl_client_v17__getunconfirmedbalance!();
crate::impl_client_v17__getwalletinfo!();
crate::impl_client_v17__listaddressgroupings!();
crate::impl_client_v17__listlabels!();
crate::impl_client_v17__listlockunspent!();
crate::impl_client_v17__listreceivedbyaddress!();
crate::impl_client_v17__listsinceblock!();
crate::impl_client_v17__listtransactions!();
crate::impl_client_v17__listunspent!();
crate::impl_client_v17__listwallets!();
crate::impl_client_v17__loadwallet!();
crate::impl_client_v17__rescanblockchain!();
crate::impl_client_v17__sendmany!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__signmessage!();
crate::impl_client_v17__signrawtransactionwithwallet!();
crate::impl_client_v17__unloadwallet!();
crate::impl_client_v17__walletcreatefundedpsbt!();
crate::impl_client_v17__walletprocesspsbt!();

/// Argument to the `Client::get_new_address_with_type` function.
///
/// For Core versions 0.17 through to v22. For Core v23 and onwards use `v23::AddressType`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddressType {
    Legacy,
    P2shSegwit,
    Bech32,
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AddressType::*;

        let s = match *self {
            Legacy => "legacy",
            P2shSegwit => "p2sh-segwit",
            Bech32 => "bech32",
        };
        fmt::Display::fmt(s, f)
    }
}

/// Arg for the `getblocktemplate` method.
///
/// For Core versions 0.17 through to v28. For Core v29 and onwards use `v29::TemplateRequest`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TemplateRequest {
    /// A list of strings.
    pub rules: Vec<TemplateRules>,
}

/// Client side supported softfork deployment.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateRules {
    /// SegWit v0 supported.
    Segwit,
    /// Signet supported.
    Signet,
    /// CSV supported.
    Csv,
    /// Taproot supported.
    Taproot,
}

/// Input used as parameter to `create_raw_transaction`.
#[derive(Debug, Serialize)]
pub struct Input {
    /// The txid of the transaction that contains the UTXO.
    pub txid: bitcoin::Txid,
    /// The vout for the UTXO.
    pub vout: u64,
    /// Sequence number if needed.
    pub sequence: Option<bitcoin::Sequence>,
}

/// Output used as parameter to `create_raw_transaction`.
// Abuse `HashMap` so we can derive serialize to get the correct JSON object.
#[derive(Debug, Serialize)]
pub struct Output(
    /// Map of address to value. Always only has a single item in it.
    HashMap<String, f64>,
);

impl Output {
    /// Creates a single output that serializes as Core expects.
    pub fn new(addr: Address, value: Amount) -> Self {
        let mut map = HashMap::new();
        map.insert(addr.to_string(), value.to_btc());
        Output(map)
    }
}

/// An element in the `inputs` argument of method `walletcreatefundedpsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtInput {
    txid: Txid,
    vout: u32,
}

/// Args for the `addnode` method
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AddNodeCommand {
    Add,
    Remove,
    OneTry,
}

/// Args for the `setban` method
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SetBanCommand {
    Add,
    Remove,
}
