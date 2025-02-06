use crate::core::types::Block;
use crate::core::types::Address;
use crate::core::consensus::ConsensusEngine;

pub fn get_current_validator(consensus: &mut ConsensusEngine) -> Option<Address> {
    crate::core::consensus::select_next_validator(consensus)
}

pub fn get_latest_block(consensus: &mut ConsensusEngine) -> Option<Block> {
    consensus.chain.last().cloned()
}
