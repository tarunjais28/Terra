use super::*;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Encode, Decode)]
pub struct Proposal {
    pub status: ProposalStatus,
    pub yes_votes: u128,
    pub yes_votes_total: u8,
    pub proposed_block: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Encode, Decode)]
pub enum ProposalStatus {
    Inactive,
    Active,
    Passed,
    Executed,
    Cancelled,
}
