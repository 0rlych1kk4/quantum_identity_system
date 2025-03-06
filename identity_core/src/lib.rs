use pqcrypto_ntru::ntruhps2048509::{keypair as ntru_keypair, encrypt as ntru_encrypt, decrypt as ntru_decrypt};
use pqcrypto_dilithium::dilithium2::{keypair as dilithium_keypair, sign as dilithium_sign, verify as dilithium_verify};
use serde::{Serialize, Deserialize};
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize, Debug)]
pub struct QuantumIdentity {
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

impl QuantumIdentity {
    pub fn new() -> Self {
        let (pk, sk) = dilithium_keypair();
        let signature = dilithium_sign(b"Quantum Identity", &sk);
        Self {
            public_key: pk.as_bytes().to_vec(),
            signature: signature.as_bytes().to_vec(),
        }
    }

    pub fn verify(&self) -> bool {
        dilithium_verify(b"Quantum Identity", &self.signature, &self.public_key).is_ok()
    }
}

