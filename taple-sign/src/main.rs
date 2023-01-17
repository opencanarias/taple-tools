use chrono::Utc;
use clap::Parser;
use commons::{
    crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair, Payload, DSA},
    identifier::{Derivable, DigestIdentifier, KeyIdentifier, SignatureIdentifier},
};
use core::{
    event_request::{EventRequestType, StateRequest},
    signature::{Signature, SignatureContent},
};
use core::{
    ExternalEventRequestBody, SignatureRequest, SignatureRequestContent, StateRequestBody,
    StateRequestBodyUpper,
};
use std::str::FromStr;

#[derive(Parser, Default, Debug)]
#[clap(
    version,
    about = "TAPLE requests generator utility for invokation to TAPLE nodes"
)]
struct Args {
    /// Private key to use. HEX String format
    private_key: String,
    /// JSON String of the request data to sign
    request: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let key_bytes = hex::decode(args.private_key)?;
    let key_pair = KeyPair::Ed25519(Ed25519KeyPair::from_secret_key(&key_bytes));
    let request = {
        if args.request.starts_with('\'') || args.request.ends_with('\'') {
            remove_first_and_last_characters(&args.request)
        } else {
            args.request
        }
    };
    let request_data: StateRequestBody = serde_json::from_str(&request)?;
    let request = EventRequestType::State(StateRequest {
        subject_id: DigestIdentifier::from_str(&request_data.subject_id)?,
        payload: request_data.payload.clone().into(),
    });
    let timestamp = Utc::now().timestamp_millis();
    let signature: Signature = sign(&key_pair, request, timestamp)?;
    let external_request = ExternalEventRequestBody {
        request: StateRequestBodyUpper {
            State: request_data,
        },
        timestamp,
        signature: SignatureRequest {
            content: SignatureRequestContent {
                signer: signature.content.signer.clone().to_str(),
                event_content_hash: signature.content.event_content_hash.clone().to_str(),
                timestamp: signature.content.timestamp,
            },
            signature: signature.signature.to_str(),
        },
    };
    let result: String = serde_json::to_string_pretty(&external_request)?;
    println!("{}", result);
    Ok(())
}

fn sign(
    keys: &KeyPair,
    data: EventRequestType,
    timestamp: i64,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let hash = DigestIdentifier::from_serializable_borsh((data, timestamp))?;
    let signature = keys.sign(Payload::Buffer(hash.derivative()))?;
    let identifier = generate_identifier(&keys);
    Ok(Signature {
        content: SignatureContent {
            signer: identifier.clone(),
            event_content_hash: hash,
            timestamp: Utc::now().timestamp_millis(),
        },
        signature: SignatureIdentifier::new(identifier.to_signature_derivator(), &signature),
    })
}

fn generate_identifier(keys: &KeyPair) -> KeyIdentifier {
    KeyIdentifier::new(keys.get_key_derivator(), &keys.public_key_bytes())
}

fn remove_first_and_last_characters(s: &str) -> String {
    s.chars().skip(1).take(s.len() - 2).collect()
}
