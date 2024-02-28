use bitcoin::public_hash_function;

fn main() {
	let public_key = "12345";
	let hash = public_hash_function(public_key.as_bytes());
	println!("{:?}", hash);
}
