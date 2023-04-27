use chrono::Utc;
use clap::Parser;
use taple_core::TimeStamp;
use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair, Payload, DSA};
use taple_core::identifier::{Derivable, DigestIdentifier, KeyIdentifier, SignatureIdentifier};

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
    let request: EventRequestTypeBody = serde_json::from_str(&request)?;
    let timestamp = Utc::now().timestamp_millis();
    let signature: Signature = sign(&key_pair, &request, timestamp)?;

    let external_request = EventRequest {
        request,
        timestamp: TimeStamp { time: timestamp as u64 },
        signature,
    };
    let result: String = serde_json::to_string_pretty(&external_request)?;
    println!("{}", result);
    Ok(())
}

fn sign(
    keys: &KeyPair,
    data: &EventRequestTypeBody,
    timestamp: i64,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let hash = DigestIdentifier::from_serializable_borsh(&(data, timestamp))?;
    let signature = keys.sign(Payload::Buffer(hash.derivative()))?;
    let identifier = generate_identifier(&keys);
    Ok(Signature {
        content: SignatureContent {
            signer: identifier.to_str(),
            event_content_hash: hash.to_str(),
            timestamp: TimeStamp::now(),
        },
        signature: SignatureIdentifier::new(identifier.to_signature_derivator(), &signature).to_str(),
    })
}

fn generate_identifier(keys: &KeyPair) -> KeyIdentifier {
    KeyIdentifier::new(keys.get_key_derivator(), &keys.public_key_bytes())
}

fn remove_first_and_last_characters(s: &str) -> String {
    s.chars().skip(1).take(s.len() - 2).collect()
}
