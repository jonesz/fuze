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
    fn approx<const N: usize>(bba: impl IntoIterator<Item = (S, T)>) -> [Option<(S, T)>; N];
}

pub struct KX();

impl<S: Set> Approximation<S, f32> for KX {
    fn approx<const N: usize>(bba: impl IntoIterator<Item = (S, f32)>) -> [Option<(S, f32)>; N] {
        // Utilize a PH to capture the N largest elements within the BBA.
        let mut container = PriorityHeap::<N, (S, f32)>::default();
        bba.into_iter().for_each(|x| {
            let f = |x: &(S, f32)| x.1;
            container.insert_by_key(f, x);
        });

        // Rescale so that the resulting BBA sums to `1.0f32`.
        let mut buf = container.consume();
        let denom: f32 = buf.iter().flatten().map(|e| e.1).sum();
        buf.iter_mut().flatten().for_each(|mem| mem.1 /= denom);

        buf
    }
}

pub struct Summarize();

impl<S: Set> Approximation<S, f32> for Summarize {
    fn approx<const N: usize>(bba: impl IntoIterator<Item = (S, f32)>) -> [Option<(S, f32)>; N] {
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

        let mut buf = container.consume();
        // Back to N vs (N-1) -- merge the `merged` into the last element of the arr.
        if let Some(mem) = buf.iter_mut().find(|x| x.is_none()) {
            *mem = Some(merged);
        } else {
            // TODO: It might be wiser to find the smallest element, which isn't guaranteed
            // to be the last element.
            let last = buf.get(N - 1).unwrap().as_ref().unwrap();
            merged = (S::cup(&merged.0, &last.0), merged.1 + last.1);
            *buf.get_mut(N - 1).unwrap() = Some(merged);
        }

        buf
    }
}

#[cfg(test)]
mod tests {

    mod kx {
        use super::super::{Approximation, KX};

        #[test]
        fn test_kx_full() {
            let input = [(1usize, 0.25f32), (2, 0.50f32), (3, 0.25f32)];
            for elem in KX::approx::<3>(input).iter().flatten() {
                assert!(input.contains(&elem));
            }
        }

        #[test]
        fn test_kx_overflow() {
            let input = [(1usize, 0.25f32), (2, 0.20f32), (3, 0.25f32), (4, 0.30f32)];
            let output = [
                (1usize, 0.25f32 / 0.8f32),
                (3, 0.25f32 / 0.8f32),
                (4, 0.30f32 / 0.8f32),
            ];

            for elem in KX::approx::<3>(input).iter().flatten() {
                assert!(output.contains(&elem));
            }
        }

        #[test]
        fn test_kx_incomplete() {
            let input = [(1usize, 0.50f32), (3, 0.50f32)];
            let output = [(1usize, 0.50f32), (3, 0.50f32), (0, 0.0f32)];

            for elem in KX::approx::<3>(input).iter().flatten() {
                assert!(output.contains(&elem));
            }
        }
    }

    mod summarize {
        use super::super::{Approximation, Summarize};

        #[test]
        fn test_summarize_full() {
            let input = [(1usize, 0.25f32), (2, 0.50f32), (3, 0.25f32)];
            for elem in Summarize::approx::<3>(input).iter().flatten() {
                assert!(input.contains(&elem));
            }
        }

        #[test]
        fn test_summarize_overflow() {
            let input = [(1usize, 0.10f32), (2, 0.20f32), (3, 0.30f32), (4, 0.40f32)];
            let output = [(4usize, 0.40f32), (3, 0.30f32), (1 | 2, 0.10f32 + 0.20f32)];

            for elem in Summarize::approx::<3>(input).iter().flatten() {
                assert!(output.contains(&elem));
            }
        }

        #[test]
        fn test_summarize_incomplete() {
            let input = [(1usize, 0.50f32), (3, 0.50f32)];
            let output = [(1usize, 0.50f32), (3, 0.50f32), (0, 0.0f32)];

            for elem in Summarize::approx::<3>(input).iter().flatten() {
                assert!(output.contains(&elem));
            }
        }
    }
}
