use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use taple_core::{
    event_request::CreateRequest as TCreateRequest, event_request::StateRequest as TStateRequest,
    event_request::TransferRequest as TTreansferRequest, DigestIdentifier, EventRequestType,
    KeyIdentifier, TimeStamp,
};

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum EventRequestTypeBody {
    Create(CreateRequest),
    State(StateRequest),
    Transfer(TransferRequest),
}

impl Into<EventRequestType> for EventRequestTypeBody {
    fn into(self) -> EventRequestType {
        match self {
            Self::Create(data) => EventRequestType::Create(TCreateRequest {
                governance_id: DigestIdentifier::from_str(&data.governance_id)
                    .expect("Should be DigestIdentifier"),
                schema_id: data.schema_id,
                namespace: data.namespace,
            }),
            Self::State(data) => EventRequestType::State(TStateRequest {
                subject_id: DigestIdentifier::from_str(&data.subject_id)
                    .expect("Should be DigestIdentifier"),
                invokation: data.invokation,
            }),
            Self::Transfer(data) => EventRequestType::Transfer(TTreansferRequest {
                subject_id: DigestIdentifier::from_str(&data.subject_id)
                    .expect("Should be DigestIdentifier"),
                public_key: KeyIdentifier::from_str(&data.subject_pub_key)
                    .expect("Should be KeyIdentifier"),
            }),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct TransferRequest {
    pub subject_id: String,
    pub subject_pub_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub content: SignatureContent,
    pub signature: String, // SignatureIdentifier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureContent {
    pub signer: String,
    pub event_content_hash: String,
    pub timestamp: TimeStamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRequest {
    pub request: EventRequestTypeBody,
    pub timestamp: TimeStamp,
    pub signature: Signature,
}
