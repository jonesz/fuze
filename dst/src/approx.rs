//! Approximation schemes within Dempster-Shafer Theory.
//!
//! Consider multiple BBAs with multiple arbitrary elements: `[f][?](set, f32)`;
//! beyond the computational infeasibility of producing combinations, it's
//! difficult to do this in parallel because of potential arr irregularity.
//! With an approximation, we can take some BBA `[?](set, f32)` and reduce it
//! down to a known-length `[k](set, f32)`, allowing for reduce and so forth.

use crate::container::heap::PriorityHeap;
use crate::set::Set;

pub trait Approximation<S: Set, T> {
    fn approx<const N: usize>(bba: impl IntoIterator<Item = (S, T)>) -> [(S, T); N];
}

pub struct KX();

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
        let mut buf = core::array::from_fn(|_| container_iter.next().unwrap_or((S::EMPTY, 0.0f32)));

        // Rescale so that the resulting BBA sums to `1.0f32`.
        let denom: f32 = buf.iter().map(|e| e.1).sum();
        buf.iter_mut().for_each(|mem| mem.1 /= denom);

        buf
    }
}

pub struct Summarize();

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
        let mut buf = core::array::from_fn(|_| container_iter.next().unwrap_or((S::EMPTY, 0.0f32)));

        // Back to N vs (N-1) -- merge the `merged` into the last element of the arr.
        let last = buf.get(N - 1).unwrap();
        merged = (S::cup(&merged.0, &last.0), merged.1 + last.1);
        *buf.get_mut(N - 1).unwrap() = merged;

        buf
    }
}
