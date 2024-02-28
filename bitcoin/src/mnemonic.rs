use sha2::Sha512;
use hmac::Hmac;
use pbkdf2::pbkdf2;


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
    pub fn to_seed(&self) -> Vec<u8> {
        let mnemonic = self.mnemonic.as_bytes();
        let salt = format!("mnemonic{}", self.password.unwrap_or(""));
        let mut seed = vec![0u8; PBKDF2_BYTES];
        pbkdf2::pbkdf2::<Hmac<Sha512>>(
            mnemonic,
            salt.as_bytes(),
            PBKDF2_ROUNDS,
            &mut seed,
        ).expect("HMAC can not be initialized with any key length");
        seed
    }
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
}