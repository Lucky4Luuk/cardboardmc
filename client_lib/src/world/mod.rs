use std::collections::HashMap;

pub mod block;
pub mod chunk;

use block::{Block, BlockId, BlockPos};
use chunk::{ChunkPos, Chunk};

pub struct World {
    block_palette: HashMap<BlockId, Block>,

    loaded_chunks: HashMap<ChunkPos, Chunk>,
}

impl World {
    pub fn empty() -> Self {
        Self {
            block_palette: HashMap::new(),
            loaded_chunks: HashMap::new(),
        }
    }

    pub fn register_block(&mut self, block_id: BlockId, block: Block) {
        if self.block_palette.insert(block_id, block).is_some() {
            warn!("Registered block that already was registered!");
        }
    }

    // TODO: Panics when trying to place a block in an unloaded chunk. Perhaps handle this error?
    // TODO: Panics when trying to place a block who isn't yet loaded into the palette. Perhaps handle this error?
    pub fn write_block(&mut self, chunk_pos: ChunkPos, block_pos: BlockPos, block_id: BlockId) {
        if !self.loaded_chunks.contains_key(&chunk_pos) {
            panic!("Trying to place block in unloaded chunk!");
        }

        if !self.block_palette.contains_key(&block_id) {
            panic!("Trying to place block that has not been loaded yet!");
        }

        let chunk = self.loaded_chunks.get_mut(&chunk_pos).unwrap();
        chunk.write_block(block_id, block_pos);
    }
}
