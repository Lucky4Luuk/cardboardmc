pub type BlockId = usize;
pub type BlockPos = (u8, i16, u8); // TODO: Put in a single u32, might be more optimized?

#[derive(Copy, Clone)]
pub struct Block {
    pub id: BlockId,
}
