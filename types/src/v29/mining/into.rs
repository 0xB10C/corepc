// SPDX-License-Identifier: CC0-1.0

use bitcoin::hex::FromHex as _;
use bitcoin::{
    block, consensus, BlockHash, CompactTarget, SignedAmount, Target, Transaction, Txid, Weight,
    Wtxid,
};

use super::{
    BlockTemplateTransaction, BlockTemplateTransactionError, GetBlockTemplate,
    GetBlockTemplateError, GetMiningInfo, GetMiningInfoError, NextBlockInfo, NextBlockInfoError,
};
use crate::model;

impl GetBlockTemplate {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockTemplate, GetBlockTemplateError> {
        use GetBlockTemplateError as E;

        let version = block::Version::from_consensus(self.version);
        let version_bits_required =
            crate::to_u32(self.version_bits_required, "version_bits_required")?;
        let previous_block_hash =
            self.previous_block_hash.parse::<BlockHash>().map_err(E::PreviousBlockHash)?;
        let transactions = self
            .transactions
            .into_iter()
            .map(|t| t.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Transactions)?;
        let coinbase_value = SignedAmount::from_sat(self.coinbase_value);
        let target = Vec::from_hex(&self.target).map_err(E::Target)?;
        let sigop_limit = crate::to_u32(self.sigop_limit, "sigop_limit")?;
        let weight_limit = crate::to_u32(self.weight_limit, "weight_limit")?;
        let size_limit = crate::to_u32(self.size_limit, "size_limit")?;
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let height = crate::to_u32(self.height, "height")?;

        Ok(model::GetBlockTemplate {
            version,
            rules: self.rules,
            version_bits_available: self.version_bits_available,
            version_bits_required,
            previous_block_hash,
            transactions,
            coinbase_aux: self.coinbase_aux,
            coinbase_value,
            target,
            min_time: self.min_time,
            mutable: self.mutable,
            nonce_range: self.nonce_range,
            sigop_limit,
            weight_limit,
            size_limit,
            current_time: self.current_time,
            bits,
            height,
        })
    }
}

impl BlockTemplateTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::BlockTemplateTransaction, BlockTemplateTransactionError> {
        use BlockTemplateTransactionError as E;

        let data =
            consensus::encode::deserialize_hex::<Transaction>(&self.data).map_err(E::Data)?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.hash.parse::<Wtxid>().map_err(E::Hash)?;
        let depends = self
            .depends
            .iter()
            .map(|x| crate::to_u32(*x, "depend"))
            .collect::<Result<Vec<_>, _>>()?;
        let fee = SignedAmount::from_sat(self.fee);
        let sigops = crate::to_u32(self.sigops, "sigops")?;
        let weight = Weight::from_wu(self.weight); // FIXME: Is this the correct unit?

        Ok(model::BlockTemplateTransaction { data, txid, wtxid, depends, fee, sigops, weight })
    }
}

impl GetMiningInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMiningInfo, GetMiningInfoError> {
        use GetMiningInfoError as E;

        let current_block_weight = self.current_block_weight.map(Weight::from_wu);
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let target = Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?;

        let next = self.next.into_model().map_err(E::Next)?;

        Ok(model::GetMiningInfo {
            blocks: self.blocks,
            current_block_weight,
            current_block_tx: self.current_block_tx,
            bits: Some(bits),
            difficulty: self.difficulty,
            target: Some(target),
            network_hash_ps: self.network_hash_ps,
            pooled_tx: self.pooled_tx,
            chain: self.chain,
            signet_challenge: self.signet_challenge,
            next: Some(next),
            warnings: self.warnings,
        })
    }
}

impl NextBlockInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::NextBlockInfo, NextBlockInfoError> {
        use NextBlockInfoError as E;

        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let target = Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?;

        Ok(model::NextBlockInfo { height: self.height, bits, difficulty: self.difficulty, target })
    }
}
