//! Utilities for computing a Cartesian Product.
//!
//! For some BBAs, we need to compute their Cartesian Product;
//! included are utilities for computing said Cartesian Product.

// TODO: Does this potentially need to be named as some sort
// `CartesianProductIterator` to indicate that it is an
// iterator?
#[derive(Clone, Debug)]
pub struct CartesianProduct<'a, const D: usize, I> {
    // We don't particularly need to utilize an iterator; we're
    // going to be iterating over predictions produced by some model,
    // so the data should reside in memory already.
    // TODO: What if this is coming over the network? Would abstracting
    // it as an iterator allow for that use case?
    items: [&'a [I]; D],

    // TODO: This technically doesn't have to be stored; we could
    // call the `.len()` on the slice (or potential iter) and
    // drop this off the stack.
    lengths: [usize; D],
    indices: [usize; D],

    consumed: bool,
}

impl<'a, const D: usize, I> CartesianProduct<'a, D, I> {
    pub fn new(items: [&'a [I]; D]) -> Self {
        let indices = [0usize; D];
        let mut lengths = [0usize; D];
        for (item, length) in items.iter().zip(lengths.iter_mut()) {
            *length = item.len();
        }

        Self {
            items,
            lengths,
            indices,
            consumed: false,
        }
    }

    // Helper function to bump the appropriate index.
    fn inc(&mut self, idx: usize) -> Result<(), ()> {
        if let Some(to_inc) = self.indices.get_mut(idx) {
            *to_inc += 1;
            let length = self.lengths.get(idx).unwrap();

            if *to_inc >= *length {
                *to_inc = 0;
                self.inc(idx + 1)
            } else {
                Ok(())
            }
        } else {
            self.consumed = true;
            Err(())
        }
    }
}

impl<const D: usize, I> Iterator for CartesianProduct<'_, D, I>
where
    I: Copy,
{
    type Item = [I; D];

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            None
        } else {
            let mut buf: [I; D] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
            for (bba_idx, indices_idx) in self.indices.iter().enumerate() {
                let mem = buf.get_mut(bba_idx).unwrap();
                let val = self
                    .items
                    .get_mut(bba_idx)
                    .unwrap()
                    .get(*indices_idx)
                    .unwrap();

                *mem = *val;
            }

            let _ = self.inc(0usize);
            Some(buf)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_single() {
        let items: [&[usize]; 3] = [&[1], &[3, 4, 5, 6], &[1, 2, 3]];
        let mut product = CartesianProduct::<3, usize>::new(items);

        // A single increment should have incremented the second index.
        assert!(product.inc(0).is_ok());
        assert_eq!(product.indices[0], 0);
        assert_eq!(product.indices[1], 1);
        assert_eq!(product.indices[2], 0);
    }

    #[test]
    fn test_inc_multiple() {
        let items: [&[usize]; 3] = [&[1], &[3, 4, 5, 6], &[1, 2, 3]];
        let mut product = CartesianProduct::<3, usize>::new(items);

        for _ in 0..6 {
            assert!(product.inc(0).is_ok());
        }
        assert_ne!(product.indices[2], 0);
    }

    #[test]
    fn test_inc_consuming() {
        let items: [&[usize]; 3] = [&[1], &[3, 4, 5, 6], &[1, 2, 3]];
        let mut product = CartesianProduct::<3, usize>::new(items);

        // The indices of the first value have been produced (0, 0, ...), so
        // the total is `-1`.
        let total_items = (1 * 4 * 3) - 1;
        for _ in 0..total_items {
            assert!(product.inc(0).is_ok());
        }

        assert!(product.inc(0).is_err());
    }

    #[test]
    fn test_iterator() {
        let items: [&[usize]; 3] = [&[1], &[3, 4, 5, 6], &[1, 2, 3]];
        let mut product = CartesianProduct::<3, usize>::new(items);

        // Iterate over all values within the second arr.
        assert_eq!(product.next().unwrap(), [1usize, 3, 1]);
        assert_eq!(product.next().unwrap(), [1usize, 4, 1]);
        assert_eq!(product.next().unwrap(), [1usize, 5, 1]);
        assert_eq!(product.next().unwrap(), [1usize, 6, 1]);

        // Roll over the last arr.
        assert_eq!(product.next().unwrap(), [1usize, 3, 2]);
    }

    #[test]
    fn test_iterator_consumed() {
        let items: [&[usize]; 3] = [&[1], &[3, 4, 5, 6], &[1, 2, 3]];
        let mut product = CartesianProduct::<3, usize>::new(items);

        // The total items isn't -1 here; we effectively return
        // the last computed value.
        let total_items = 1 * 4 * 3;
        for _ in 0..total_items {
            assert!(product.next().is_some());
        }

        assert!(product.next().is_none());
    }
}
