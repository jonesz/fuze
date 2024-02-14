mod pq {

    pub struct BoundedPriorityQueue<T, const N: usize> {
        buf: [T; N],

        // `buf` at the time of writing is uninitialized with MaybeUninit.
        // We keep track of how may times insert has been called; once `N`
        // has been eclipsed, all indices of `buf` are guaranteed to be
        // initialized.
        num_initialized: usize,
    }

    impl<T, const N: usize> BoundedPriorityQueue<T, N> {
        pub fn new() -> Self {
            let buf: [T; N] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
            Self {
                buf,
                num_initialized: 0,
            }
        }

        pub fn insert_by_key<F, R>(&mut self, x: T, f: F)
        where
            T: Copy,
            R: Ord,
            F: Fn(&T) -> R,
        {
            // Compute the index for where `x` should reside.
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
                return N.checked_sub(1);
            };

            // If we haven't inserted N values, we insert regardless. If we've inserted N,
            // we need to sort the underlying arr. And if we've inserted > N values, our
            // insertion works like a normal priority queue.
            if self.num_initialized < N {
                let mem = self.buf.get_mut(self.num_initialized).unwrap();
                *mem = x;

                self.num_initialized += 1;
                if self.num_initialized == N {
                    self.buf.sort_unstable_by_key(f);
                }
            } else {
                // If this `x` value belongs in the structure, we find the index it resides at,
                // then push all of those values to the left, dropping the smallest value.
                if let Some(idx) = index() {
                    for i in 0..idx {
                        let r_mem = *self.buf.get(i + 1).unwrap();
                        let l_mem = self.buf.get_mut(i).unwrap();
                        *l_mem = r_mem;
                    }

                    let mem = self.buf.get_mut(idx).unwrap();
                    *mem = x;
                }
            }
        }

        pub fn insert(&mut self, x: T)
        where
            T: Ord + Copy,
        {
            let f = |x: &T| *x;
            self.insert_by_key(x, f)
        }
    }

    impl<T, const N: usize> IntoIterator for BoundedPriorityQueue<T, N> {
        type Item = T;
        type IntoIter = core::array::IntoIter<T, N>;

        // TODO: Handle this case when we haven't triggered `num_initialized`, or the `num_initialized` sort?
        // An option is to just panic.
        fn into_iter(self) -> Self::IntoIter {
            self.buf.into_iter()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_i32() {
            const N: usize = 3;

            let mut s: BoundedPriorityQueue<i32, N> = BoundedPriorityQueue::new();
            let mut a = [-5i32, 5i32, 0i32, -2i32, 2i32, 1i32, -99i32, 100i32];
            for b in a.into_iter() {
                s.insert(b);
            }

            // We place the largest values at the front with `sort` into `reverse`;
            // We have to then reverse the iterator after taking `N`: the BPQ stores
            // values from smallest to largest.
            a.sort();
            a.reverse();
            for (a, b) in a.into_iter().take(N).rev().zip(s.into_iter()) {
                assert_eq!(a, b);
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
        *mem = *bba.get(j).unwrap();
    }

    todo!("Normalize.");

    kx
}
