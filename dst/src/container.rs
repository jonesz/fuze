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

/// Heap-oriented structures.
pub mod heap {

    /// A Bounded Priority Heap.
    #[derive(Debug)]
    pub struct PriorityHeap<const N: usize, T> {
        buf: [Option<T>; N],
    }

    impl<const N: usize, T> Default for PriorityHeap<N, T> {
        fn default() -> Self {
            Self {
                buf: core::array::from_fn(|_| None),
            }
        }
    }

    impl<const N: usize, T> PriorityHeap<N, T> {
        /// The index at which leaves begin.
        const LEAF_IDX: usize = usize::ilog2(N) as usize;

        /// Compute the parent of a child with the array representation of a heap.
        const fn parent(x: usize) -> usize {
            assert!(x != 0); // We should never call this function on the root.
            match x % 2 {
                0 => (x - 2) / 2,
                1 => (x - 1) / 2,
                _ => unreachable!(),
            }
        }

        /// Assert that the heap condition holds for the child `idx`.
        fn heap_upkeep<R: PartialOrd>(&mut self, f: impl Fn(&T) -> R, child_idx: usize) {
            // If the `child_idx` is the root, then the heap condition holds and we exit.
            if child_idx == 0 {
                return;
            }

            let parent_idx = Self::parent(child_idx);

            let p = self.buf.get(parent_idx).unwrap();
            let c = self.buf.get(child_idx).unwrap();

            // If the child is larger than the parent, we need to swap them.
            if f(c.as_ref().unwrap()) > f(p.as_ref().unwrap()) {
                self.buf.swap(parent_idx, child_idx);
                // Assert that the condition holds for the *new* parent index.
                self.heap_upkeep(f, parent_idx);
            };
        }

        // TODO: `impl Fn(&T) -> R`: this constrains the API to things that can `Copy`.
        /// Insert a value into the Heap, returning the value that was ejected.
        pub fn insert_by_key<R: PartialOrd>(&mut self, f: impl Fn(&T) -> R, v: T) -> Option<T> {
            // If there's a `None`, attempt to find it and replace it.
            let (idx, r) = if let Some((idx, mem)) =
                self.buf.iter_mut().enumerate().find(|(_, x)| x.is_none())
            {
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

                (idx + Self::LEAF_IDX, mem.replace(v)) // We started at LEAF_IDX, add it back...
            };

            self.heap_upkeep(f, idx);
            r
        }

        /// Return the underyling buffer.
        pub fn consume(self) -> [Option<T>; N] {
            self.buf
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const N: usize = 4;
        type PH = PriorityHeap<N, usize>;

        #[test]
        fn test_parent() {
            assert_eq!(PH::parent(1), 0);
            assert_eq!(PH::parent(2), 0);
            assert_eq!(PH::parent(3), 1);
            assert_eq!(PH::parent(4), 1);
            //...
            assert_eq!(PH::parent(8), 3);
        }

        #[test]
        fn test_pq_simple() {
            let mut ph = PH::default();
            let f = |x: &usize| *x;

            (0..8).for_each(|x| {
                ph.insert_by_key(f, x);
            });

            // The Heap should contain 8, 7, ... 8 - N; if they sum equivalently we've
            // capture them all
            assert_eq!(
                ph.consume().iter().flatten().sum::<usize>(),
                ((8 - N)..8).sum()
            );
        }
    }
}
