use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::{Digest, Sha256, Sha512};

// Mnemonic
pub struct Mnemonic<'a> {
	mnemonic: &'a str,
	password: Option<&'a str>,
}

// Number of iterations for PBKDF2
const PBKDF2_ROUNDS: u32 = 2048;

// Length of the PBKDF2 output in bytes: 512 bits
const PBKDF2_BYTES: usize = 64;

impl Mnemonic<'_> {
	// Create a new mnemonic
	pub fn new<'a>(mnemonic: &'a str, password: Option<&'a str>) -> Mnemonic<'a> {
		Mnemonic { mnemonic, password }
	}

	// Derive the seed from the mnemonic
	pub fn to_seed(&self) -> [u8; PBKDF2_BYTES] {
		let mnemonic = self.mnemonic.as_bytes();
		let salt = format!("mnemonic{}", self.password.unwrap_or(""));
		let mut seed = [0u8; PBKDF2_BYTES];
		pbkdf2::pbkdf2::<Hmac<Sha512>>(mnemonic, salt.as_bytes(), PBKDF2_ROUNDS, &mut seed)
			.expect("HMAC can not be initialized with any key length");
		seed
	}
}

// todo: 
// 1. Move it into a new module called entropy
// 2. Add a function to generate a random mnemonic
// 3. Add test case to test from entropy to mnemonic and from mnemonic to entropy
pub fn get_checksum(input: &str) -> String {
	// Convert input to bytes
	let input = hex::decode(input).unwrap();
	
	let mut hasher = Sha256::new();
	hasher.update(input);
	let result = hasher.finalize();
	// 将结果转换为十六进制字符串
    let hashed_hex = hex::encode(result);
	println!("Hashed hex: {}", hashed_hex);
	// 获取哈希值的第一个字节
    let first_byte = result[0];

    // 将第一个字节的前4位转换为二进制字符串
    let first_4_bits = format!("{:b}", first_byte >> 4);

    // 打印前4位二进制数
    println!("First 4 bits: {}", first_4_bits);
	
	first_4_bits
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mnemonic_to_seed() {
		let mnemonic =
			"legal winner thank year wave sausage worth useful legal winner thank yellow"
				.to_string();
		let password = Some("TREZOR");
		let mnemonic_seed = Mnemonic::new(&mnemonic, password);

		let seed = mnemonic_seed.to_seed();
		assert_eq!(hex::encode(seed), "2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607")
	}

	#[test]
	fn test_get_checksum() {
		let input = "16432a207785ec5c4e5a226e3bde819d";
		let left = get_checksum(input);
		assert_eq!(left, "0111");
	}
}
