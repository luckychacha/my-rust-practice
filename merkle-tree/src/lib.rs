#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use log::info;
use sha2::{Digest, Sha256};

pub type Data = Vec<u8>;
type Hash = [u8; 32];

#[derive(Debug)]
pub struct MerkleTree {
	hash: Hash,
	left: Option<Rc<MerkleTree>>,
	right: Option<Rc<MerkleTree>>,
}

impl MerkleTree {
	fn new(hash: Hash) -> Self {
		MerkleTree {
			hash: hash,
			left: None,
			right: None,
		}
	}

	fn create_leaf(data: Data) -> Self {
		let hash = calculate_hash_digest(data);
		MerkleTree::new(hash)
	}

	fn create_branch_node(left: &Rc<MerkleTree>, right: &Rc<MerkleTree>) -> Self {
		info!("left: {:?}", left);
		info!("right: {:?}", right);
		let combined_hash = concat_and_hash(left.hash, right.hash);
		MerkleTree {
			hash: combined_hash,
			left: Some(Rc::clone(left)),
			right: Some(Rc::clone(right)),
		}
	}
}

fn calculate_hash_digest(data: Data) -> Hash {
	Sha256::digest(&data).into()
}

fn concat_and_hash(left: Hash, right: Hash) -> Hash {
	let mut data = Vec::with_capacity(left.len() + right.len());
	data.extend_from_slice(&left);
	data.extend_from_slice(&right);
	calculate_hash_digest(data)
}

#[cfg(test)]
mod test {

use super::*;

	#[test]
	fn test_create_leaf() {
		env_logger::init();

		let data = b"hello world".to_vec();
		let leaf = MerkleTree::create_leaf(data);
		assert_eq!(leaf.hash.len(), 32);
	}

	#[test]
	fn test_create_branch_node() {
		env_logger::init();

		let left = MerkleTree::create_leaf(b"hello".to_vec());
		let right = MerkleTree::create_leaf(b"world".to_vec());
		let branch = MerkleTree::create_branch_node(&Rc::new(left), &Rc::new(right));
		assert_eq!(branch.hash.len(), 32);
	}
}