use crate::prelude::*;
use crate::block::Block;

struct Blockchain {
	blocks: Vec<Block>
}

impl Blockchain {
	pub fn new() -> Result<Self, MiningError> {
		Ok(Self {
			blocks: vec![Block::genesis()?]
		})
	}

	pub fn add_block(&mut self, payload: &str) -> Result<(), MiningError> {
		self.blocks.push(
			match self.blocks.last() {
				Some(prev) => {
					Block::new(prev.hash(), payload)?
				},
				None => {
					return Err(MiningError::NoParent);
				}
			}
		);

		Ok(())
	}
}