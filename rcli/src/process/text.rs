use std::{fs, io::Read, path::{Path}};

use base64::{prelude::BASE64_STANDARD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{get_reader, TextSignFormat};

use super::process_genpass;


pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;

}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(blake3::keyed_hash(&self.key, &buffer).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        Ok(hash.as_bytes() == sig)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = ed25519_dalek::SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().as_bytes().to_vec();
        let sk = sk.as_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Blake3::new(key))
    }

    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }

}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }

}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let signature = self.key.sign(&buffer);
        Ok(signature.to_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key_bytes = fs::read(path)?;
        Self::try_new(&key_bytes)
    }

}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let signature = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buffer, &signature).is_ok())
    }

}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key_bytes = fs::read(path)?;
        Self::try_new(&key_bytes)
    }

}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Ed25519Signer { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..64]; // Ed25519 keys are 64 bytes
        let key = SigningKey::from_bytes(&key.try_into()?);
        Ok(Ed25519Signer::new(key))
    }

}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Ed25519Verifier { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = VerifyingKey::from_bytes(&key.try_into()?)?;
        Ok(Ed25519Verifier::new(key))
    }

}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    // Simulate signing process
    let _signed_data = match format {
        TextSignFormat::Blake3 => {
            let blake3_signer = Blake3::load(key)?;
            blake3_signer.sign(&mut reader)?
        },
        TextSignFormat::Ed25519 => {
            let ed25519_signer = Ed25519Signer::load(key)?;
            ed25519_signer.sign(&mut reader)?
        },
    };
    let signed = BASE64_STANDARD.encode(&_signed_data);
    println!("{}", signed);
    Ok(())
}

pub fn process_verify(input: &str, key: &str, sig: &str, format: TextSignFormat) -> anyhow::Result<bool> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    // Simulate verification process
    let is_valid = match format {
        TextSignFormat::Blake3 => {
            let blake3_verifier = Blake3::load(key)?;
            blake3_verifier.verify(&mut reader, &BASE64_STANDARD.decode(sig)?)?
        },
        TextSignFormat::Ed25519 => {
            let ed25519_verifier = Ed25519Verifier::load(key)?;
            ed25519_verifier.verify(&mut reader, &BASE64_STANDARD.decode(sig)?)?
        },
    };

    println!("Verification result: {}", is_valid);
    Ok(is_valid)
}

pub fn process_generate(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate()
    }
}
