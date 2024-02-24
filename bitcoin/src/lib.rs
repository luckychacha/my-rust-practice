use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

// public hash function
// input: public key
// hashed by SHA256 and ripemd160
// output: hash
pub fn public_hash_function(public_key: &[u8]) -> Vec<u8> {
    let hasher = Sha256::digest(public_key);
    let hash256: [u8; 32] = hasher.try_into().expect("Wrong length");
    let hasher = Ripemd160::digest(hash256);
    let hash160: [u8; 20] = hasher.try_into().expect("Wrong length");
    hash160.to_vec()
}