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
                    .map(|opt| f(opt.as_ref().expect(ERROR_NONE))) // Compute `R: PartialOrd` for each &T.
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

        pub fn consume(self) -> [Option<T>; N] {
            self.buf
        }
    }
}

struct Summarize;
struct KX;

impl<S, T> Approximation<S, T> for Summarize
where
    T: PartialOrd,
{
    fn approx<'a, const N: usize, I: IntoIterator<Item = &'a (S, T)> + Clone>(x: I) -> [(S, T); N]
    where
        S: 'a + Clone,
        T: 'a + Clone,
    {
        //let _iter_dup = x.clone();

        //let mut bpq = bpq::BoundedPriorityQueue::<(S, T), N>::default();
        //let key_fn = |x: &'a (S, T)| -> &'a T { &x.1 };

        //for elem in x.into_iter().cloned() {
        //    bpq.insert_by_key(elem, key_fn);
        //}

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

/*
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
*/
