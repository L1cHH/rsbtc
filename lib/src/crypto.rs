use std::fmt;
use std::fmt::{Debug, Formatter};
use ecdsa::{
    Signature as ECDSASignature,
    signature::Signer,
    SigningKey,
    VerifyingKey
};
use k256::{Secp256k1};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Signature(ECDSASignature<Secp256k1>);


impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ecdsa::Signature<{:?}>(", Secp256k1::default())?;

        for byte in self.0.to_bytes() {
            write!(f, "{:02X}", byte)?;
        }

        write!(f, ")")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

#[derive(Serialize, Deserialize, Clone)]
pub struct PrivateKey(
    #[serde(with = "signkey_serde")]
    pub SigningKey<Secp256k1>);

///Used for generating PubKey
impl PrivateKey {
    ///
    pub fn new_key() -> Self {
        PrivateKey(SigningKey::random(&mut rand::thread_rng()))
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(VerifyingKey::from(self.0.clone()))
    }
}

mod signkey_serde {
    use serde::Deserialize;
    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey<super::Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> =
            Vec::<u8>::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}

