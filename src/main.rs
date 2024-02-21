use sha2::{Digest, Sha256, Sha512};
use hex_literal::hex;
fn main() {
    let hash = Sha256::digest(b"Hello, world!");
    println!("{:x}", hash);

    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let _hash256 = hasher.finalize();


    let mut hasher = Sha512::new();
    hasher.update(b"hello world");
    let _hash512 = hasher.finalize();

}
