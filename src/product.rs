#[derive(Clone, Debug)]
struct CartesianProduct<'a, const D: usize, I> {
    items: [&'a [I]; D],
    lengths: [usize; D],
    indices: [usize; D],

    consumed: bool,
}

impl<'a, const D: usize, I> CartesianProduct<'a, D, I> {
    pub fn new(items: [&'a [I]; D]) -> Self {
        let indices = [0usize; D];
        let mut lengths = [0usize; D];
        for (idx, (item, length)) in items.iter().zip(lengths.iter_mut()).enumerate() {
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
            *to_inc = *to_inc + 1;
            let length = self.lengths.get(idx).unwrap();

            if *to_inc >= *length {
                *to_inc = 0;
                return self.inc(idx + 1);
            } else {
                return Ok(());
            }
        } else {
            self.consumed = true;
            Err(())
        }
    }
}

impl<const D: usize, I> Iterator for CartesianProduct<'_, D, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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

        for i in 0..6 {
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
}
