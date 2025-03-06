use wasm_bindgen::prelude::*;
use identity_core::QuantumIdentity;

#[wasm_bindgen]
pub fn generate_identity() -> String {
    let identity = QuantumIdentity::new();
    serde_json::to_string(&identity).unwrap()
}

#[wasm_bindgen]
pub fn verify_identity(identity_json: &str) -> bool {
    let identity: QuantumIdentity = serde_json::from_str(identity_json).unwrap();
    identity.verify()
}

