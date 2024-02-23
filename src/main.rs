use sha2::{Digest, Sha256, Sha512};
use hex_literal::hex;
fn main() {
    let hash = Sha256::digest(b"Hello, world!");
    println!("{:x}", hash);
    // let a  = "Hello, world!".as_bytes();
    // hex! 宏主要用于将十六进制字符串转换为字节数组
    println!("{:?}", b"Hello, world!");
    
    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let hash256: [u8; 32] = hasher.finalize().try_into().expect("Wrong length");

    assert_eq!(hash256, hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));

}
