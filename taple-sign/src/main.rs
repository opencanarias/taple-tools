use chrono::Utc;
use clap::Parser;
use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair, Payload, DSA};
use taple_core::identifier::{Derivable, DigestIdentifier, KeyIdentifier, SignatureIdentifier};
use taple_core::{EventRequestType, TimeStamp};

mod model;
use model::*;
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
    let request_body: EventRequestTypeBody = serde_json::from_str(&request)?;
    let request: EventRequestType = request_body.clone().into();

    let signature: Signature = sign(&key_pair, &request)?;

    let external_request = EventRequest {
        request: request_body,
        signature,
    };
    let result: String = serde_json::to_string_pretty(&external_request)?;
    println!("{}", result);
    Ok(())
}

fn sign(keys: &KeyPair, data: &EventRequestType) -> Result<Signature, Box<dyn std::error::Error>> {
    let content_hash = DigestIdentifier::from_serializable_borsh(&data)?;
    let timestamp = TimeStamp::now();
    let signature_hash = DigestIdentifier::from_serializable_borsh((&content_hash, &timestamp))?;
    let signature = keys.sign(Payload::Buffer(signature_hash.derivative()))?;
    let identifier = generate_identifier(&keys);
    Ok(Signature {
        content: SignatureContent {
            signer: identifier.to_str(),
            event_content_hash: content_hash.to_str(),
            timestamp,
        },
        signature: SignatureIdentifier::new(identifier.to_signature_derivator(), &signature)
            .to_str(),
    })
}

fn generate_identifier(keys: &KeyPair) -> KeyIdentifier {
    KeyIdentifier::new(keys.get_key_derivator(), &keys.public_key_bytes())
}

fn remove_first_and_last_characters(s: &str) -> String {
    s.chars().skip(1).take(s.len() - 2).collect()
}
