use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use crate::crypto::bls::{BlsSignature, BlsPublicKey};

pub fn encode_signature(sig: &BlsSignature) -> String {
    URL_SAFE_NO_PAD.encode(sig.to_bytes())
}

pub fn decode_signature(s: &str) -> Result<BlsSignature, crate::serialization::error::DeserializationError> {
    let bytes: Vec<u8> = URL_SAFE_NO_PAD.decode(s).map_err(|e| {
        crate::serialization::error::DeserializationError::Schema(e.to_string())
    })?;
    let bytes_array: [u8; 96] = bytes.try_into().map_err(|_| {
        crate::serialization::error::DeserializationError::Schema("Invalid signature length".to_string())
    })?;
    Ok(BlsSignature::from_bytes(&bytes_array))
}

pub fn encode_public_key(pk: &BlsPublicKey) -> String {
    URL_SAFE_NO_PAD.encode(pk.to_bytes())
}
