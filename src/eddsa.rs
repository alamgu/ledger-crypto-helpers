use arrayvec::{ArrayVec};
use nanos_sdk::ecc::*;
use nanos_sdk::io::SyscallError;

use crate::common::*;

#[derive(Clone,Debug,PartialEq)]
pub struct EdDSASignature(pub [u8; 64]);

pub type Ed25519PublicKey = ECPublicKey<65, 'E'>;

pub fn eddsa_sign(
    path : &ArrayVec<u32, 10>,
    m: &[u8],
) -> Result<EdDSASignature, CryptographyError> {
    let sig = Ed25519::from_bip32(path).sign(m)?;
    Ok(EdDSASignature(sig.0))
}

pub fn with_public_keys<V, A:Address<A, Ed25519PublicKey>>(
  path: &[u32],
  f: impl FnOnce(&nanos_sdk::ecc::ECPublicKey<65, 'E'>, &A) -> Result<V, CryptographyError>
) -> Result<V, CryptographyError> {
    let pubkey = Ed25519::from_bip32(path).public_key()?;
    let pkh = <A as Address<A, Ed25519PublicKey>>::get_address(&pubkey)?;
    f(&pubkey, &pkh)
}

pub struct Ed25519RawPubKeyAddress(nanos_sdk::ecc::ECPublicKey<65, 'E'>);

impl Address<Ed25519RawPubKeyAddress, nanos_sdk::ecc::ECPublicKey<65, 'E'>> for Ed25519RawPubKeyAddress {
    fn get_address(key: &nanos_sdk::ecc::ECPublicKey<65, 'E'>) -> Result<Self, SyscallError> {
        Ok(Ed25519RawPubKeyAddress(key.clone()))
    }
    fn get_binary_address(&self) -> &[u8] {
        ed25519_public_key_bytes(&self.0)
    }
}
impl core::fmt::Display for Ed25519RawPubKeyAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", HexSlice(&self.0.pubkey[1..self.0.keylength]))
    }
}

pub fn ed25519_public_key_bytes(key: &Ed25519PublicKey) -> &[u8] {
    &key.pubkey[1..33]
}
