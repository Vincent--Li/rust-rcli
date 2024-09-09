use std::{fs, io::Read, path::Path};

use crate::{get_reader, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use super::process_genpass;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: impl Read, signature: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>
    where
        Self: Sized;
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
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

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = BASE64_URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let signature = BASE64_URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &signature)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &signature)?
        }
    };
    println!("{}", verified);
    Ok(())
}

pub fn process_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == signature)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(true, true, true, true, 32)?;
        let key = key.as_bytes().to_ascii_lowercase();
        Ok(vec![key])
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = Signature::from_bytes(signature.try_into()?);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3 { key };
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(&key),
        }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Ed25519Signer {
            key: SigningKey::from_bytes(&key),
        };
        Ok(signer)
    }
}

impl Ed25519Verifier {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key: VerifyingKey::from_bytes(&key).unwrap(),
        }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let verifier = Ed25519Verifier {
            key: VerifyingKey::from_bytes(&key)?,
        };
        Ok(verifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() {
        let blake3 = Blake3::load("fixtures/blake3.txt").unwrap();
        let data = "hello world";
        let signature = blake3.sign(&mut data.as_bytes()).unwrap();
        println!("signature: {}", BASE64_URL_SAFE_NO_PAD.encode(&signature));
        assert!(blake3.verify(&mut data.as_bytes(), &signature).unwrap());
    }

    #[test]
    fn test_ed25519_sign_verify() {
        // load sk pk
        let sk = Ed25519Signer::load("fixtures/ed25519.sk").unwrap();
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk").unwrap();
        let data = "hello world";
        let signature = sk.sign(&mut data.as_bytes()).unwrap();
        println!("signature: {}", BASE64_URL_SAFE_NO_PAD.encode(&signature));
        assert!(pk.verify(&mut data.as_bytes(), &signature).unwrap());
    }
}
