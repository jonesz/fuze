#[derive(Clone, Debug)]
pub struct CartesianProduct<'a, const D: usize, I> {
    items: [&'a [I]; D],
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
        let total_items = (1 * 4 * 3);
        for _ in 0..total_items {
            assert!(product.next().is_some());
        }

        assert!(product.next().is_none());
    }
}
