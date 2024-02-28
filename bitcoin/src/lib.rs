use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
pub mod mnemonic;

// public hash function
// input: public key
// hashed by SHA256 and ripemd160
// output: hash
pub fn public_hash_function(public_key: &[u8]) -> [u8; 20] {
	let sha256_digest = Sha256::digest(public_key);
	let sha256_digest: [u8; 32] = sha256_digest.try_into().expect("Wrong length");
	let ripemd160_digest = Ripemd160::digest(sha256_digest);
	let res: [u8; 20] = ripemd160_digest.try_into().expect("Wrong length");
	// res.to_vec()
	res
}

#[cfg(test)]
mod tests {
	use super::*;
	use hex::FromHex;
	use hex_literal::hex;

	#[test]
	fn test_from_sha256_to_ripemd160() {
		// hex! is a macro that converts a string literal to a byte array
		// so we can not use hex! to deal a parameter, we should use hex::FromHex.
		let _sha256 = "85ae273f0aa730eddf2285d3f3ab071eb29caba1e428db90e6dfbd71b8e1e918";
		let _ripemd160 = "5f2613791b36f667fdb8e95608b55e3df4c5f9eb";
		let hasher = Ripemd160::digest(hex!(
			"85ae273f0aa730eddf2285d3f3ab071eb29caba1e428db90e6dfbd71b8e1e918"
		));
		let l: [u8; 20] = hasher.try_into().expect("Wrong length");
		let r = hex!("5f2613791b36f667fdb8e95608b55e3df4c5f9eb");
		assert_eq!(l, r);
	}

	#[test]
	fn test_from_sha256_to_ripemd160_use_hex() {
		// We can use hex::FromHex to deal a parameter.
		let sha256 = "85ae273f0aa730eddf2285d3f3ab071eb29caba1e428db90e6dfbd71b8e1e918";
		let ripemd160 = "5f2613791b36f667fdb8e95608b55e3df4c5f9eb";
		let hasher = Ripemd160::digest(Vec::<u8>::from_hex(sha256).expect("Decoding failed"));
		let l: [u8; 20] = hasher.try_into().expect("Wrong length");
		let r = Vec::<u8>::from_hex(ripemd160).expect("Decoding failed");
		assert_eq!(l, r.as_slice());
	}
}
