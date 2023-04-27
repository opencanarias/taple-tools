use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};
use taple_core::TimeStamp;


#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum EventRequestTypeBody {
    Create(CreateRequest),
    State(StateRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct CreateRequest {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct StateRequest {
    pub subject_id: String,
    pub invokation: String,
}

#[derive(
    Debug, Clone, Serialize, Deserialize,
)]
pub struct Signature {
    pub content: SignatureContent,
    pub signature: String, // SignatureIdentifier
}

#[derive(
    Debug, Clone, Serialize, Deserialize,
)]
pub struct SignatureContent {
    pub signer: String,
    pub event_content_hash: String,
    pub timestamp: TimeStamp,
}

#[derive(
  Debug, Clone, Serialize, Deserialize,
)]
pub struct EventRequest {
    pub request: EventRequestTypeBody,
    pub timestamp: TimeStamp,
    pub signature: Signature,
}
