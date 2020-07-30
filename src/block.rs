/// This module contains the implementation for a single block in the blockchain

// libark types
use crate::prelude::*;

// chrono
use chrono::prelude::*;

// cypto
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use num_bigint::BigUint;
use num_traits::One;

const DIFFICULTY: usize = 5;
const MAX_NONCE: u64 = 1000000;

pub struct Block {
	pub timestamp: i64,
	pub prev_hash: Sha256Hash,
	pub payload: PayloadContent,
	pub nonce: u64,
}

/// Block implmentation
impl Block {
	/// Creates a new block with the current time
	pub fn new(
		prev_hash: Sha256Hash,
		payload: &str
	) -> Result<Self, MiningError> {
		let mut block = Self {
			timestamp: Utc::now().timestamp(),
			prev_hash: prev_hash,
			payload: payload.to_owned().into(),
			nonce: 0,
		};

		block.nonce = match block.try_hash() {
			Some(nonce) => { nonce },

			None => {
				return Err(MiningError::Iteration); 
			}
		};

		Ok(block)
	}

	/// Creates the genesis block
	pub fn genesis() -> Result<Self, MiningError> {
		Self::new(Sha256Hash::default(), "Genesis")
	}

	fn try_hash(&self) -> Option<u64> {
		let target = BigUint::one() << (256 - 4 * DIFFICULTY);
		for i in 0..MAX_NONCE {
			let hash = Block::calculate_hash(&self, i);
			let hash_int = BigUint::from_bytes_be(&hash);
			
			if hash_int < target {
				return Some(i);
			}
		}

		None
	}

	pub fn calculate_hash(block: &Block, nonce: u64) -> Sha256Hash {
		let mut headers = block.headers();
		headers.extend_from_slice(&convert_u64_to_u8_array(nonce));

		let mut hasher = Sha256::new();
		hasher.input(&headers);
		let mut hash = Sha256Hash::default();

		hasher.result(&mut hash);

		hash
	}

	pub fn hash(&self) -> Sha256Hash {
		let mut headers = self.headers();
		headers.extend_from_slice(&convert_u64_to_u8_array(self.nonce));

		let mut hasher = Sha256::new();
		hasher.input(&headers);
		let mut hash = Sha256Hash::default();

		hasher.result(&mut hash);

		hash
	}

	pub fn headers(&self) -> PayloadContent {
		let mut data: PayloadContent = PayloadContent::new();
		data.extend(&convert_u64_to_u8_array(self.timestamp as u64));
		data.extend_from_slice(&self.prev_hash);
		
		data
	}
}

pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
	let mut out: [u8; 8] = [0; 8];

	for i in 0..7 {
		out[i] = (val >> (i*8)) as u8;
	}

	out
}