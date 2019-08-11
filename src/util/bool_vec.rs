//! Provides a compact collection for boolean sequence.

/// An array-like sequence of boolean bits.
///
/// As compared to an array of booleans, this sequence
/// is eight times more space-efficient.
///
/// The functionality is implemented using a trait so
/// that any the algorithm is generic with repect to
/// the backing container's length and any allocation
/// scheme may be used, even a static array of fixed
/// length.
pub trait BoolVec {
    /// Returns the boolean representation of the bit at the given index.
    fn bit(&self, index: usize) -> bool;
    
    /// Assigns the truth value to the bit at the given index.
    fn set_bit(&mut self, index: usize, value: bool);
}

impl BoolVec for [u8] {
    fn bit(&self, index: usize) -> bool {
        let (bkt, rem) = bucket_and_inner_index(index);
        
        let bucket = self[bkt];
        let isolated = bucket & (1 << rem);
        
        isolated != 0
    }
    
    fn set_bit(&mut self, index: usize, value: bool) {
        let (bkt, rem) = bucket_and_inner_index(index);
        
        let mask = 1 << rem;
        
        if value {
            self[bkt] |= mask;
        } else {
            self[bkt] &= !mask;
        }
    }
}

/// Returns a tuple containing the bucket index
/// and the inner bit index. Bit indices are least
/// siginificant bit lowest.
fn bucket_and_inner_index(index: usize) -> (usize, usize) {
    let bkt = index / 8;
    let rem = index % 8;
    
    (bkt, rem)
}
