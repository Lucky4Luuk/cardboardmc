use std::collections::HashMap;

use super::block::{BlockId, BlockPos};

pub type ChunkPos = (usize, usize);

pub struct Chunk {
    /// Stores blocks in chunk
    blocks: HashMap<BlockPos, BlockId>,
}

impl Chunk {
    pub fn empty() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn write_block(&mut self, block_id: BlockId, pos: BlockPos) {
        self.blocks.insert(pos, block_id);
    }

    pub fn read_block(&self, pos: BlockPos) -> Option<BlockId> {
        self.blocks.get(&pos).map(|id| *id)
    }
}
