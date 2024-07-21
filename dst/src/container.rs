//! Containers needed for our Dempster-Shafer impls.

/// "Exhaustive Map" -- a map where we find keys via exhaustive search.
pub(super) mod em {
    use core::ops::{AddAssign, MulAssign};

    /// A map that when `insert` is called the values are summed; keys are found via
    /// exhaustive search.
    pub struct SummationEM<const N: usize, K, V> {
        // TODO: We need `generic_const_expr` to compute `N * N`.
        // buf: [Option<(K, V)>; N * N],
        // ... but a [[...; N]; N] dimension arr is potentially a way to avoid this.
        buf: [[Option<(K, V)>; N]; N],
    }

    impl<const N: usize, K, V> Default for SummationEM<N, K, V> {
        fn default() -> Self {
            // TODO: If `(K, V)` is `Copy` this becomes `[[None; N]; N]` which is much,
            // much nicer. There's also opportunities for `MaybeUninit`? This buffer should
            // just be zeroed (assuming that stable Rust zeroes `None`).
            let buf: [[Option<(K, V)>; N]; N] =
                core::array::from_fn(|_| core::array::from_fn(|_| None));
            Self { buf }
        }
    }

    impl<const N: usize, K, V> SummationEM<N, K, V>
    where
        K: PartialEq,
        V: AddAssign + MulAssign,
    {
        /// Insert a `(K, V)` pair into the map, summing the `V` value if found.
        pub fn insert(&mut self, k: K, v: V) {
            let mem = self
                .buf
                .iter_mut()
                .flatten()
                // We're looking for a `None` or the index where the keys match, so DeMorgan's...
                .find(|z| !z.as_ref().is_some_and(|y| y.0 != k))
                .expect("Should have found an index to place this KV in; is N correct?.");

            if let Some(inner) = mem {
                inner.1 += v;
            } else {
                mem.replace((k, v));
            }
        }

        pub fn scale(&mut self, s: V)
        where
            V: Copy, // See the below TODO and the commit info.
        {
            self.buf
                .iter_mut()
                .flatten()
                .flatten()
                // TODO: The `*= s` triggers a `Copy` of `s`; what's the
                // perf impact? We could utilize a reference, but it makes the
                // trait bounds uglier.
                .for_each(|x| x.1 *= s);
        }

        // TODO: Think about `IntoIter` rather than this?
        /// Return an iterator over the underlying buffer.
        pub fn consume(self) -> impl Iterator<Item = (K, V)> {
            // [[...; N]; N] -> [...; N * N] alongside dumping all `None` options.
            self.buf.into_iter().flatten().flatten()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_insert() {
            let mut shm = SummationEM::<3, usize, usize>::default();
            shm.insert(0, 10);
            shm.insert(1, 20);
            shm.insert(0, 30);

            let mut iter = shm.consume();
            assert_eq!(iter.next(), Some((0, 40)));
            assert_eq!(iter.next(), Some((1, 20)));
            assert!(iter.next().is_none());
        }
    }
}

pub struct PriorityQueue<const N: usize, T> {
    buf: [Option<T>; N],
}

impl<const N: usize, T> Default for PriorityQueue<N, T> {
    fn default() -> Self {
        Self {
            buf: core::array::from_fn(|_| None),
        }
    }
}

impl<const N: usize, T> PriorityQueue<N, T> {
    const LEAF_IDX: usize = usize::ilog2(N) as usize;

    /// Compute the children of the passed index with the array representation of a heap.
    const fn children(x: usize) -> (usize, usize) {
        (x * 2 + 1, x * 2 + 2)
    }

    /// Compute the parent of a child.
    const fn parent(x: usize) -> usize {
        todo!()
    }

    fn heap_upkeep(&mut self, idx: usize) {
        todo!();
    }

    pub fn insert_by_key<R: PartialOrd>(&mut self, f: impl Fn(&T) -> &R, v: T) -> Option<T> {
        // If there's a `None`, attempt to find it and replace it.
        let (idx, r) =
            if let Some((idx, mem)) = self.buf.iter_mut().enumerate().find(|(_, x)| x.is_none()) {
                (idx, mem.replace(v))
            } else {
                // Otherwise, find the minimum value then replace it.
                let (idx, mem) = self.buf[Self::LEAF_IDX..]
                    .iter_mut()
                    .enumerate()
                    .reduce(|(a, min), (b, e)| {
                        if f(min.as_ref().unwrap()) > f(e.as_ref().unwrap()) {
                            (b, e)
                        } else {
                            (a, min)
                        }
                    })
                    .unwrap();
                (idx, mem.replace(v))
            };

        self.heap_upkeep(idx);
        r
    }

    pub fn consume(self) -> impl Iterator<Item = T> {
        self.buf.into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type PQ = PriorityQueue<16, usize>;

    #[test]
    fn test_children() {
        assert_eq!(PQ::children(0), (1, 2));
        assert_eq!(PQ::children(1), (3, 4));
        assert_eq!(PQ::children(2), (5, 6));
        assert_eq!(PQ::children(3), (7, 8));
    }
}
