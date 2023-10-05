#![deny(unsafe_code)]

pub mod graffiti;

use graffiti::Graffiti;

pub type Signature = String;

use crate::{Epoch, Hash256, Slot, Uint256};

// randao_reveal eth1_data graffiti proposer_slashings attester_slashings attestations deposits voluntary_exits sync_aggregate execution_payload
pub struct BeaconBlockBody {
    pub randao_reveal: String,
    pub eth1_data: Eth1Data,
    pub graffiti: Graffiti,
    pub proposer_slashings: Vec<u8>,
    pub attester_slashings: Vec<u8>,
    pub attestations: Vec<Attestation>,
    pub deposits: Vec<u8>,
    pub voluntary_exits: Vec<u8>,
    pub sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload,
}

pub struct Eth1Data {
    pub deposit_root: Hash256,
    pub deposit_count: Uint256,
    pub block_hash: Hash256,
}

pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: Hash256,
}

pub struct AttestationData {
    pub slot: Slot,
    pub index: u64,
    pub beacon_block_root: Hash256,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

pub struct Attestation {
    pub aggregation_bits: String,
    pub data: AttestationData,
    pub signature: Signature,
}

pub struct SyncAggregate {
    pub sync_committee_bits: String,
    pub sync_committee_signature: Signature,
}

pub struct ExecutionPayload {
    pub parent_hash: Hash256,
    pub fee_recipient: String,
}

pub struct BeaconBlock {
    pub slot: Slot,
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body: BeaconBlockBody,
}

pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: Signature,
}
