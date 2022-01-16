// ANCHOR: ConstBlockSize
pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
// ANCHOR_END: ConstBlockSize

// ANCHOR: ConstLineSize
pub const LINE_SIZE_BITS: usize = 7;
pub const LINE_SIZE: usize = 1 << LINE_SIZE_BITS;
pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;
// ANCHOR_END: ConstLineSize
