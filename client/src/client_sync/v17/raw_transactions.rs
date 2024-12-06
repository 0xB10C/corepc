// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `sendrawtransaction`
#[macro_export]
macro_rules! impl_client_v17__sendrawtransaction {
    () => {
        impl Client {
            pub fn send_raw_transaction(
                &self,
                tx: &bitcoin::Transaction,
            ) -> Result<SendRawTransaction> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                self.call("sendrawtransaction", &[hex.into()])
            }
        }
    };
}
