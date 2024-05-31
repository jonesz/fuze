//! Combination rules for Dempster-Shafer Theory.
use crate::approx::Approximation;
use crate::set::SetOperations;
use core::marker::PhantomData;

mod store {
    use core::ops::{AddAssign, MulAssign};

    /// A linear store, where insertions/gets are started from the zero'th
    /// position.
    pub(super) struct LinearStore<const N: usize, K, V> {
        buf: [Option<(K, V)>; N],
    }

    impl<const N: usize, K, V> Default for LinearStore<N, K, V>
    where
        V: Copy,
    {
        fn default() -> Self {
            Self {
                buf: core::array::from_fn(|_| None),
            }
        }
    }

    impl<const N: usize, K, V> LinearStore<N, K, V>
    where
        K: Eq,
        V: MulAssign + AddAssign + Copy,
    {
        /// Insert a key-value pair into the store.
        pub fn insert(&mut self, k: K, v: V) {
            let mem = self
                .buf
                .iter_mut()
                // The iterator is sequential (0, 1, ...), so `None` will occur after all
                // `Some(x)` have been encountered.
                .find(|opt| opt.is_none() || opt.as_ref().is_some_and(|(a, _)| &k == a))
                .expect("Unable to find a position; `N` const likely wrong.");

            // Within `Option`, there's no clean API to `insert` here without the branch.
            // TODO: Attempt to find one.
            if let Some((_, m)) = mem {
                *m += v;
            } else {
                *mem = Some((k, v));
            }
        }

        /// Get the associated value for a passed key.
        pub fn get(&self, k: &K) -> Option<&V> {
            self.buf.iter().find_map(|opt| {
                opt.as_ref() // `find_map` returns the first `Some(b)` encountered.
                    .and_then(|(a, b)| if k == a { Some(b) } else { None })
            })
        }

        /// Get the underlying buffer for this struct.
        pub fn buf(&self) -> &[Option<(K, V)>; N] {
            &self.buf
        }

        /// Scale the entire vector by some value.
        pub fn scale(&mut self, v: V) {
            self.buf.iter_mut().flatten().for_each(|(_, m)| {
                *m *= v;
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const N: usize = 8;

        // Construct the default store for testing.
        fn default_store() -> LinearStore<N, usize, usize> {
            let mut store = LinearStore::<N, usize, usize>::default();
            store.insert(0, 1);
            store.insert(1, 2);
            store.insert(2, 4);
            store
        }

        #[test]
        fn test_insert_get() {
            let mut store = default_store();

            assert!(store.get(&0).is_some());
            assert_eq!(store.get(&0).unwrap(), &1);

            assert!(store.get(&3).is_none());
            assert_eq!(store.get(&3), None);

            store.insert(0, 10);
            assert!(store.get(&0).is_some());
            assert_eq!(store.get(&0).unwrap(), &11);

            store.insert(4, 5);
            assert!(store.get(&4).is_some());
            assert_eq!(store.get(&4).unwrap(), &5);
        }
    }
}

pub trait CombRule<S: SetOperations, T> {
    // TODO: We need 'const generic exprs' in stable to avoid the N2 constraint...
    /// Combine a set of BBAs where we initially compute an approximation, and then after each combination
    /// `m1 comb m2` we compute an approximation.
    fn comb<'a, const N: usize, const N2: usize, A>(
        bba: impl IntoIterator<Item = impl IntoIterator<Item = &'a (S, T)> + Clone>,
    ) -> [(S, T); N]
    where
        A: Approximation<S, T>,
        S: 'a,
        T: 'a;
}

pub struct Dempster<S, T>(PhantomData<S>, PhantomData<T>);

impl<S> CombRule<S, f32> for Dempster<S, f32>
where
    S: SetOperations + Copy, // TODO: Get rid of the `Copy`.
{
    fn comb<'a, const N: usize, const N2: usize, A>(
        bba: impl IntoIterator<Item = impl IntoIterator<Item = &'a (S, f32)> + Clone>,
    ) -> [(S, f32); N]
    where
        A: Approximation<S, f32>,
        S: 'a,
    {
        // TODO: See the comment within the trait about 'const generic exprs'. There's `N * N`
        // intersections to compute between each subset after we compute the initial approximation
        // (&[(S, f32)] -> [(S, f32); N]); these have to be placed on the stack...
        // Below function is effectively an unitialized arr (TODO: Maybe we should use `MaybeUninit`?).
        assert!(N2 == N * N);

        bba.into_iter()
            .map(|e| A::approx(e)) // Compute the initial approximations.
            .reduce(|acc, e: [(S, f32); N]| {
                let mut store = store::LinearStore::<N2, S, f32>::default();
                let mut k = 0.0f32; // Running conflict when B \cap C is empty.

                for (acc_i, e_i) in acc.iter().flat_map(|x1| e.iter().map(move |x2| (x1, x2))) {
                    // B \cap C = m1(B) * m2(C) where B \cap C != \empty.
                    let a_cap_e = acc_i.0.intersection(&e_i.0);
                    if a_cap_e.is_empty() {
                        k += acc_i.1 * e_i.1;
                    } else {
                        store.insert(a_cap_e, acc_i.1 * e_i.1);
                    }
                }

                // Compute the conflict \frac{1}{1-K} and then scale the arr.
                let conflict = 1f32 / (1f32 - k);
                store.scale(conflict);

                // Compute the approximation.
                A::approx(store.buf().iter().flatten())
            })
            .expect("Zero-length iterator passed in?")
    }
}
