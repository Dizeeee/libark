// [Hash as bytes, hash size in bytes]
pub type Sha256Hash = [u8; 32];
pub type PayloadContent = Vec<u8>;

pub enum MiningError {
	Iteration,
	NoParent,
}