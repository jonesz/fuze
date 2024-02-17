//! Approximation schemes within Dempster-Shafer Theory.
//!
//! Consider multiple BBAs with multiple arbitrary elements: `[f][?](set, f32)`;
//! beyond the computational infeasibility of producing combinations, it's
//! difficult to do this in parallel because of potential arr irregularity.
//! With an approximation, we can take some BBA `[?](set, f32)` and reduce it
//! down to a known-length `[k](set, f32)`, allowing for reduce and so forth.

/// PQ structures that are useful for keeping track of the `k` largest elements.
mod pq {

    /// A PQ that is backed by a known-length arr.
    pub struct BoundedPriorityQueue<T, const N: usize> {
        buf: [T; N],

        // `buf` at the time of writing is uninitialized with MaybeUninit.
        // We keep track of how may times insert has been called; once `N`
        // has been eclipsed, all indices of `buf` are guaranteed to be
        // initialized.
        num_initialized: usize,
    }

    impl<T, const N: usize> BoundedPriorityQueue<T, N> {
        /// Create a new `BoundedPriorityQueue` that is effectively uninitialized.
        pub fn new() -> Self {
            let buf: [T; N] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
            Self {
                buf,
                num_initialized: 0,
            }
        }

        /// Insert a `T` into the queue by a key extraction function.
        pub fn insert_by_key<F, R>(&mut self, x: T, f: F)
        where
            T: Copy,
            R: Ord,
            F: Fn(&T) -> R,
        {
            // Compute the index for where the parent `x` should reside.
            // TODO: this function should *always* return an index `< N`, but the index
            // computation in this function and the below if/else feels brittle.
            let index = || -> Option<usize> {
                for (i, v) in self.buf.iter().enumerate() {
                    if f(&x) > f(v) {
                        continue;
                    } else {
                        // If this underflows, then `x` is smaller than the smallest value -- thus `None`.
                        return i.checked_sub(1);
                    }
                }

                // The entire for loop completed; thus `x` is the largest value.
                Some(N - 1)
            };

            // If we haven't inserted `N` values, we insert regardless; then if we've inserted `N`,
            // we need to sort the underlying arr. And if we've inserted `> N` values, our
            // insertion works like a normal priority queue.
            if self.num_initialized < N {
                let mem = self.buf.get_mut(self.num_initialized).unwrap();
                *mem = x;

                self.num_initialized += 1;
                if self.num_initialized == N {
                    self.buf.sort_unstable_by_key(f);
                }
            } else {
                // We find the index which `x` should slot into (if it's too small, `None`),
                // then push all of those values to the left.
                if let Some(idx) = index() {
                    for i in 0..idx {
                        let r_mem = *self.buf.get(i + 1).unwrap(); // TODO: Fix this brittle indexing.
                        let l_mem = self.buf.get_mut(i).unwrap();
                        *l_mem = r_mem;
                    }

                    let mem = self.buf.get_mut(idx).unwrap();
                    *mem = x;
                }
            }
        }

        /// Insert a `T` into the queue.
        pub fn insert(&mut self, x: T)
        where
            T: Ord + Copy,
        {
            let f = |x: &T| *x; // TODO: I'm assuming this computes to a no-op?
            self.insert_by_key(x, f)
        }
    }

    /// `IntoIter` for a reference to a BPQ.
    // The benefit to doing it this way is that we can return a slice; this allows
    // us to cut off those potentially uninitialized values.
    impl<'a, T, const N: usize> IntoIterator for &'a BoundedPriorityQueue<T, N> {
        type Item = &'a T;
        type IntoIter = core::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            if self.num_initialized < N {
                self.buf[0..self.num_initialized].iter()
            } else {
                self.buf.as_slice().iter()
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_insert() {
            const N: usize = 3;
            let test_range = -10..10;
            let mut bpq_n: BoundedPriorityQueue<i32, N> = BoundedPriorityQueue::new();

            for v in test_range.clone() {
                bpq_n.insert(v);
            }

            // The BPQ should have the N largest values (which are the 3 at the end of the range).
            let iter = test_range.into_iter().rev().take(N).rev();
            for (a, b) in iter.zip(bpq_n.into_iter()) {
                assert_eq!(a, *b);
            }
        }

        #[test]
        fn test_insert_by_key() {
            const N: usize = 3;
            let test_range = (0..20usize).zip(-10..10i32);
            let mut bpq_n: BoundedPriorityQueue<(usize, i32), N> = BoundedPriorityQueue::new();

            let f = |a: &(usize, i32)| a.1;
            for v in test_range.clone() {
                bpq_n.insert_by_key(v, f);
            }

            // The BPQ should have the N largest values (which are the 3 at the end of the range).
            let iter = test_range.rev().take(N).rev();
            for (a, b) in iter.zip(bpq_n.into_iter()) {
                assert_eq!(a, *b);
            }
        }

        #[test]
        fn test_uninitialized() {
            const N: usize = 50;
            let test_range = -10..10;
            let mut bpq_n: BoundedPriorityQueue<i32, N> = BoundedPriorityQueue::new();

            for v in test_range.clone() {
                bpq_n.insert(v);
            }

            // At this point, we don't `rev`: the BPQ hasn't been sorted.
            let iter = test_range.clone().into_iter();

            // The returned iterator's upper_bound should be the range length, not N.
            assert_eq!(iter.size_hint().1.unwrap(), test_range.count());
            for (a, b) in iter.zip(bpq_n.into_iter()) {
                assert_eq!(a, *b);
            }
        }
    }
}

/// Perform `summarize` resulting in `N` entires within the BBA.
pub fn summarize<const N: usize, S, T>(bba: &[(S, T)]) -> [(S, T); N]
where
    S: Copy,
    T: Ord + Copy, // TODO: Ord vs PartialOrd?
{
    // Check for the degenerate case where we have less than N elements.
    if bba.len() <= N {
        todo!("Capture the degenerate case.");
    };

    let mut bpq: pq::BoundedPriorityQueue<(usize, T), N> = pq::BoundedPriorityQueue::new();

    let bba_idx = bba.iter().enumerate().map(|(i, (_, m))| (i, *m));
    let f = |(_, m): &(usize, T)| *m;

    for x in bba_idx {
        bpq.insert_by_key(x, f);
    }

    todo!("Create the summary.");
}

/// Perform 'kx' resulting in 'N' entries within the BBA.
pub fn kx<const N: usize, S, T>(bba: &[(S, T)]) -> [(S, T); N]
where
    S: Copy,
    T: Ord + Copy, // TODO: Ord vs PartialOrd?
{
    if bba.len() <= N {
        todo!("Capture the degenerate case.");
    };

    let mut bpq: pq::BoundedPriorityQueue<(usize, T), N> = pq::BoundedPriorityQueue::new();

    let bba_idx = bba.iter().enumerate().map(|(i, (_, m))| (i, *m));
    let f = |(_, m): &(usize, T)| *m;

    for x in bba_idx {
        bpq.insert_by_key(x, f);
    }

    let mut kx: [(S, T); N] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

    for (i, (j, _)) in bpq.into_iter().enumerate() {
        let mem = kx.get_mut(i).unwrap();
        *mem = *bba.get(*j).unwrap();
    }

    todo!("Normalize.");

    kx
}
