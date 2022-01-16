use crate::constants;

/// Block marking metadata
// ANCHOR: DefBlockMeta
pub struct BlockMeta {
    line_mark: [bool; constants::LINE_COUNT],
    block_mark: bool,
}
// ANCHOR_END: DefBlockMeta

impl BlockMeta {
    /// Given a byte index into a block (the `starting_at` parameter) find the next available
    /// hole in which bump allocation can occur, or `None` if no hole can be found in this
    /// block.
    /// Takes into account conservative marking of the first unmarked line in a hole.
    // ANCHOR: DefFindNextHole
    pub fn find_next_available_hole(&self, starting_at: usize) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut start: Option<usize> = None;
        let mut stop: usize = 0;

        let starting_line = starting_at / constants::LINE_SIZE;

        for (index, marked) in self.line_mark[starting_line..].iter().enumerate() {
            let abs_index = starting_line + index;

            // count unmarked lines
            if !*marked {
                count += 1;

                // if this is the first line in a hole (and not the zeroth line), consider it
                // conservatively marked and skip to the next line
                if count == 1 && abs_index > 0 {
                    continue;
                }

                // record the first hole index
                if start.is_none() {
                    start = Some(abs_index);
                }

                // stop is now at the end of this line
                stop = abs_index + 1;
            }

            // if we reached a marked line or the end of the block, see if we have
            // a valid hole to work with
            if count > 0 && (*marked || stop >= constants::LINE_COUNT) {
                if let Some(start) = start {
                    let cursor = start * constants::LINE_SIZE;
                    let limit = stop * constants::LINE_SIZE;

                    return Some((cursor, limit));
                }
            }

            // if this line is marked and we didn't return a new cursor/limit pair by now,
            // reset the hole state
            if *marked {
                count = 0;
                start = None;
            }
        }

        None
    }
    // ANCHOR_END: DefFindNextHole
}
