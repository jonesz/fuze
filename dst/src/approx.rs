//! Approximation schemes within Dempster-Shafer Theory.
//!
//! Consider multiple BBAs with multiple arbitrary elements: `[f][?](set, f32)`;
//! beyond the computational infeasibility of producing combinations, it's
//! difficult to do this in parallel because of potential arr irregularity.
//! With an approximation, we can take some BBA `[?](set, f32)` and reduce it
//! down to a known-length `[k](set, f32)`, allowing for reduce and so forth.
use crate::set::Set;
use core::iter::Sum;
use core::ops::DivAssign;

/// An Approximation scheme that reduces the number of elements within a BBA.
pub trait Approximation<S, T> {
    /// Compute an approximation for the passed BBA.
    fn approx<'a, const N: usize, I: IntoIterator<Item = &'a (S, T)> + Clone>(x: I) -> [(S, T); N]
    where
        S: 'a + Clone,
        T: 'a + Clone;
}

/// BPQ structures that are useful for keeping track of the `k` largest elements.
mod bpq {

    /// A PQ that is backed by a known-length arr.
    pub struct BoundedPriorityQueue<T, const N: usize> {
        buf: [Option<T>; N],
        initialized: usize,
    }

    impl<T, const N: usize> Default for BoundedPriorityQueue<T, N> {
        fn default() -> Self {
            Self {
                buf: core::array::from_fn(|_| None),
                initialized: 0usize,
            }
        }
    }

    impl<T, const N: usize> BoundedPriorityQueue<T, N> {
        /// Insert a `T` into the queue by a key extraction function.
        pub fn insert_by_key<F, R>(&mut self, x: T, f: F)
        where
            F: Fn(&T) -> R,
            R: PartialOrd,
        {
            if self.initialized < N {
                // TODO: Technically, we know where the `None` is: it's at `self.initialized`.
                // If the structure hasn't been fully initialized, find the first `None` slot and insert `x`.
                let mem = self.buf.iter_mut().find(|opt| opt.is_none()).expect(
                    "Should have found a slot as the structure says it's not fully initialized.",
                );
                *mem = Some(x);
                self.initialized += 1;
            } else {
                const ERROR_NONE: &str = "Buffer is supposed to be fully initialized/`Some(z)`.";
                // Find the smallest value and replace it if the value to be inserted is larger.
                // utilize `reduce` to avoid needing more than a `PartialOrd` bound (f32).
                let idx = self
                    .buf
                    .iter()
                    .map(|opt| f(opt.as_ref().expect(ERROR_NONE))) // Compute `R: PartialOrd` for each `&T`.
                    .enumerate() // `(idx: usize, cmp: R)`
                    .reduce(|acc, e| if acc.1 <= e.1 { acc } else { e })
                    .expect(ERROR_NONE)
                    .0;

                // TODO: The above iteration and the below feel terrible.
                let mem = self
                    .buf
                    .get_mut(idx)
                    .expect("The returned idx should have been within `N`")
                    .as_mut()
                    .expect(
                        "The buffer should have been initialized -- this should have been Some(v)",
                    );

                if f(mem) < f(&x) {
                    *mem = x;
                }
            }
        }

        /// Return the underlying buffer.
        pub fn consume(self) -> [Option<T>; N] {
            self.buf
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const N: usize = 8;

        // BPQ requires a key fn; for usize, just dereference.
        fn key_fn_usize(x: &usize) -> usize {
            *x
        }

        fn default_bpq() -> BoundedPriorityQueue<usize, N> {
            let mut bpq = BoundedPriorityQueue::default();
            for i in 0..8 {
                bpq.insert_by_key(i, key_fn_usize);
            }

            bpq
        }

        #[test]
        fn test_insert() {
            const TEST_SET: &[usize] = &[10usize, 3, 34, 500, 66];
            let mut bpq = default_bpq();

            for i in TEST_SET.into_iter().copied() {
                bpq.insert_by_key(i, key_fn_usize);
            }

            // Of those inserted: 10, 34, 500, 66 should exist within it.
            let buf: [Option<usize>; N] = bpq.consume();

            assert!(buf.contains(&Some(10usize)));
            assert!(buf.contains(&Some(34usize)));
            assert!(buf.contains(&Some(66usize)));
            assert!(buf.contains(&Some(500usize)));

            // 3 shouldn't exist in the buf, but 4-7 from the default BPQ should.
            assert!(!buf.contains(&Some(3usize)));
            for i in 4..7 {
                assert!(buf.contains(&Some(i)));
            }
        }
    }
}

pub struct KX;

impl<S, T> Approximation<S, T> for KX
where
    S: Set + Copy,
    T: PartialOrd + Copy + Sum + From<u8> + DivAssign,
{
    // TODO: Refactor this.
    fn approx<'a, const N: usize, I: IntoIterator<Item = &'a (S, T)> + Clone>(x: I) -> [(S, T); N]
    where
        S: 'a,
        T: 'a,
    {
        // TODO: This is ripped from `comb`, should likely be refactored.
        fn build_arr<S: Set, T: From<u8>, const Z: usize>() -> [(S, T); Z] {
            core::array::from_fn(|_| (S::EMPTY, 0u8.into()))
        }

        let mut bpq = bpq::BoundedPriorityQueue::<&'a (S, T), N>::default();
        for elem in x.into_iter() {
            bpq.insert_by_key(elem, |z: &&(S, T)| -> T { z.1 });
        }

        let buf = bpq.consume();
        let bot: T = buf.iter().flatten().map(|e| e.1).sum();

        let mut out = build_arr::<S, T, N>();
        for (mem, elem) in out.iter_mut().zip(buf.iter().flatten()) {
            *mem = **elem;
            mem.1 /= bot;
        }

        out
    }
}

mod approx_rw {
    use crate::container::heap::PriorityHeap;
    use crate::set::Set;

    trait Approximation<S: Set, T> {
        fn approx<const N: usize>(bba: impl IntoIterator<Item = (S, T)>) -> [(S, T); N];
    }

    struct KX();

    impl<S: Set> Approximation<S, f32> for KX {
        fn approx<const N: usize>(bba: impl IntoIterator<Item = (S, f32)>) -> [(S, f32); N] {
            // Utilize a PH to capture the N largest elements within the BBA.
            let mut container = PriorityHeap::<N, (S, f32)>::default();
            bba.into_iter().for_each(|x| {
                let f = |x: &(S, f32)| x.1;
                container.insert_by_key(f, x);
            });

            // Push those N elements into the resulting approximation.
            let mut container_iter = container.consume();
            let mut buf =
                core::array::from_fn(|_| container_iter.next().unwrap_or((S::EMPTY, 0.0f32)));

            // Rescale so that the resulting BBA sums to `1.0f32`.
            let denom: f32 = buf.iter().map(|e| e.1).sum();
            buf.iter_mut().for_each(|mem| mem.1 /= denom);

            buf
        }
    }

    struct Summarize();

    impl<S: Set> Approximation<S, f32> for Summarize {
        fn approx<const N: usize>(bba: impl IntoIterator<Item = (S, f32)>) -> [(S, f32); N] {
            // Utilize a PH to capture the N largest elements within the BBA; those that are
            // evicted are merged together. NOTE: We technically want to store (N-1) elements,
            // but const generics make this difficult -- we'll handle this later.
            let mut container = PriorityHeap::<N, (S, f32)>::default();
            let mut merged = (S::EMPTY, 0.0f32);

            for elem in bba {
                let f = |x: &(S, f32)| x.1;
                if let Some(evicted) = container.insert_by_key(f, elem) {
                    merged = (S::cup(&merged.0, &evicted.0), merged.1 + evicted.1);
                }
            }

            // Push those N elements into the resulting approximation.
            let mut container_iter = container.consume();
            let mut buf =
                core::array::from_fn(|_| container_iter.next().unwrap_or((S::EMPTY, 0.0f32)));

            // Back to N vs (N-1) -- merge the `merged` into the last element of the arr.
            let last = buf.get(N - 1).unwrap();
            merged = (S::cup(&merged.0, &last.0), merged.1 + last.1);
            *buf.get_mut(N - 1).unwrap() = merged;

            buf
        }
    }
}
