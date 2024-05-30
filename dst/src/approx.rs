//! Approximation schemes within Dempster-Shafer Theory.
//!
//! Consider multiple BBAs with multiple arbitrary elements: `[f][?](set, f32)`;
//! beyond the computational infeasibility of producing combinations, it's
//! difficult to do this in parallel because of potential arr irregularity.
//! With an approximation, we can take some BBA `[?](set, f32)` and reduce it
//! down to a known-length `[k](set, f32)`, allowing for reduce and so forth.

use crate::set::SetOperations;

/// An Approximation scheme that reduces the number of elements within a BBA.
pub trait Approximation<S, T> {
    /// Compute an approximation for the passed BBA.
    fn approx<'a, const N: usize, I: IntoIterator<Item = &'a (S, T)> + Clone>(x: I) -> [(S, T); N]
    where
        S: 'a,
        T: 'a;
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
                // If the structure hasn't been fully initialized, find the first `None` slot and insert `x`.
                self.buf.iter_mut().find(|opt| opt.is_none()).expect(
                    "Should have found a slot as the structure says it's not fully initialized.",
                ).insert(x);
                self.initialized += 1;
            } else {
                const ERROR_NONE: &str = "Buffer is supposed to be fully initialized/`Some(z)`.";
                // Find the smallest value and replace it; utilize `reduce` to avoid needing more than a
                // `PartialOrd` bound.
                self.buf
                    .iter_mut()
                    .reduce(|acc, e| {
                        if f(acc.as_ref().expect(ERROR_NONE)) <= f(e.as_ref().expect(ERROR_NONE)) {
                            acc
                        } else {
                            e
                        }
                    })
                    .unwrap()
                    .insert(x);
            }
        }

        pub fn consume(self) -> [Option<T>; N] {
            self.buf
        }
    }
}

struct Summarize;
struct KX;

impl<S, T> Approximation<S, T> for Summarize {
    fn approx<'a, const N: usize, I: IntoIterator<Item = &'a (S, T)> + Clone>(x: I) -> [(S, T); N]
    where
        S: 'a,
        T: 'a,
    {
        let iter_dup = x.clone();
        unimplemented!();
    }
}

impl<S, T> Approximation<S, T> for KX
where
    S: SetOperations,
    T: From<u8>,
{
    fn approx<'a, const N: usize, I: IntoIterator<Item = &'a (S, T)> + Clone>(x: I) -> [(S, T); N]
    where
        S: 'a,
        T: 'a,
    {
        unimplemented!();
    }
}

/// Perform `summarize` resulting in `N` entires within the BBA.
pub fn summarize<const N: usize, S, T>(bba: &[(S, T)]) -> [(S, T); N]
where
    S: Copy + crate::set::SetOperations,
    T: Ord + Copy + From<usize> + core::iter::Sum + core::ops::Div<T, Output = T>, // TODO: Ord vs PartialOrd?
{
    let mut summarize: [(S, T); N] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

    // Check for the degenerate case where we have less than N elements.
    if bba.len() <= N {
        for (i, x) in bba.iter().enumerate() {
            let mem = summarize.get_mut(i).unwrap();
            *mem = *x;
        }

        for mem in summarize.iter_mut().skip(bba.len()) {
            *mem = (S::empty(), 0.into());
        }
    };

    let mut bpq: bpq::BoundedPriorityQueue<(usize, T), N> = bpq::BoundedPriorityQueue::new();

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
    S: Copy + crate::set::SetOperations,
    T: Ord + Copy + From<usize> + core::iter::Sum + core::ops::Div<T, Output = T>, // TODO: Ord vs PartialOrd?
{
    let mut kx: [(S, T); N] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

    // Catch the degenerate case where |bba| < `N`.
    if bba.len() < N {
        for (i, x) in bba.iter().enumerate() {
            let mem = kx.get_mut(i).unwrap();
            *mem = *x;
        }

        for mem in kx.iter_mut().skip(bba.len()) {
            *mem = (S::empty(), 0.into());
        }
    } else {
        // Otherwise, find the N largest values in the bba.
        let mut bpq: bpq::BoundedPriorityQueue<(usize, T), N> = bpq::BoundedPriorityQueue::new();

        // TODO: So there's some tradeoffs here: if we store the `usize`, the underlying BPQ will be
        // smaller, but we'll need to do another loop to copy N values. On the other hand, if we
        // push `S` we can avoid this final loop, popping them off into the arr.
        let f = |(_, m): &(usize, T)| *m;
        for x in bba.iter().enumerate().map(|(i, (_, m))| (i, *m)) {
            bpq.insert_by_key(x, f);
        }

        // Write the values to kx.
        for (i, (j, _)) in bpq.into_iter().enumerate() {
            let mem = kx.get_mut(i).unwrap();
            *mem = *bba.get(*j).unwrap();
        }

        let total: T = kx.iter().map(|(_, m)| *m).sum();

        // Normalize.
        for mem in kx.iter_mut() {
            let (s, m) = *mem;
            *mem = (s, m / total);
        }
    }

    kx
}
