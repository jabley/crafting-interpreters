use blockalloc::Block;

use crate::blockmeta::BlockMeta;
use crate::constants;

/// A block of heap. This maintains the bump cursor and limit per block
/// and the mark flags in a separate `meta` struct.  A pointer to the
/// `meta` struct is placed in the very first word of the block memory
/// to provide fast access when in the object marking phase.
/// Thus allocation in the first line of the block doesn't begin at
/// offset 0 but after this `meta` pointer.
// ANCHOR: DefBumpBlock
pub struct BumpBlock {
    cursor: usize,
    limit: usize,
    block: Block,
    meta: Box<BlockMeta>,
}
// ANCHOR_END: DefBumpBlock

impl BumpBlock {
    /// Find a hole of at least the requested size and return Some(pointer) to it, or
    /// None if this block doesn't have a big enough hole.
    // ANCHOR: DefBumpBlockAlloc
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let next_bump = self.cursor + alloc_size;

        if next_bump > self.limit {
            if self.limit < constants::BLOCK_SIZE {
                if let Some((cursor, limit)) = self.meta.find_next_available_hole(self.limit) {
                    self.cursor = cursor;
                    self.limit = limit;
                    return self.inner_alloc(alloc_size);
                }
            }

            None
        } else {
            let offset = self.cursor;
            self.cursor = next_bump;
            unsafe { Some(self.block.as_ptr().add(offset) as *const u8) }
        }
    }
    // ANCHOR_END: DefBumpBlockAlloc
}
