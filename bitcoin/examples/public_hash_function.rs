fn main() {
    // show the public hash function
    let output = bitcoin::public_hash_function("hello world".as_bytes());
    println!("{:?}", output);
}