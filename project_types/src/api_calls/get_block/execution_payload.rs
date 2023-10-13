use derivative::Derivative;
use ethereum_types::Address;
use serde_derive::{Deserialize, Serialize};
use ssz::Encode;
use ssz_derive::{Decode, Encode};
use ssz_types::{FixedVector, VariableList};
use tree_hash_derive::TreeHash;

use crate::{execution_block_hash::ExecutionBlockHash, EthSpec, Hash256, Uint256};

use super::{withdrawal::Withdrawal, Transactions};

pub type Withdrawals<T> = VariableList<Withdrawal, <T as EthSpec>::MaxWithdrawalsPerPayload>;

#[derive(
    Default,
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    Derivative,
    arbitrary::Arbitrary,
)]

pub struct ExecutionPayload<T: EthSpec> {
    pub parent_hash: ExecutionBlockHash,

    pub fee_recipient: Address,

    pub state_root: Hash256,

    pub receipts_root: Hash256,

    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    pub logs_bloom: FixedVector<u8, T::BytesPerLogsBloom>,

    pub prev_randao: Hash256,

    #[serde(with = "serde_utils::quoted_u64")]
    pub block_number: u64,

    #[serde(with = "serde_utils::quoted_u64")]
    pub gas_limit: u64,

    #[serde(with = "serde_utils::quoted_u64")]
    pub gas_used: u64,

    #[serde(with = "serde_utils::quoted_u64")]
    pub timestamp: u64,

    #[serde(with = "ssz_types::serde_utils::hex_var_list")]
    pub extra_data: VariableList<u8, T::MaxExtraDataBytes>,

    #[serde(with = "serde_utils::quoted_u256")]
    pub base_fee_per_gas: Uint256,

    pub block_hash: ExecutionBlockHash,

    #[serde(with = "ssz_types::serde_utils::list_of_hex_var_list")]
    pub transactions: Transactions<T>,

    pub withdrawals: Withdrawals<T>,
}

impl<T: EthSpec> ExecutionPayload<T> {
    pub fn max_execution_payload_size() -> usize {
        // Fixed part
        ExecutionPayload::<T>::default().as_ssz_bytes().len()
            // Max size of variable length `extra_data` field
            + (T::max_extra_data_bytes() * <u8 as Encode>::ssz_fixed_len())
            // Max size of variable length `transactions` field
            + (T::max_transactions_per_payload() * (ssz::BYTES_PER_LENGTH_OFFSET + T::max_bytes_per_transaction()))
            // Max size of variable length `withdrawals` field
            + (T::max_withdrawals_per_payload() * <Withdrawal as Encode>::ssz_fixed_len())
    }
}
