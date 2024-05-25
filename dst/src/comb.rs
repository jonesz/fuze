use crate::product;
use crate::set::SetOperations;
use core::marker::PhantomData;

mod hm {
    use core::hash::{Hash, Hasher};

    mod hm_hash {
        use super::*;
        // TODO: We could implement an actual *fast* hash (or fetch one off crates),
        // because this is within a hot loop.

        #[derive(Default)]
        pub(super) struct XorshiftHasher(u64);

        impl Hasher for XorshiftHasher {
            fn write(&mut self, bytes: &[u8]) {
                // Fold each bit in with a XorShift64 step.
                self.0 = bytes.iter().fold(self.0, |acc, elem| {
                    let mut state: u64 = acc;
                    state ^= Into::<u64>::into(*elem);
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    state
                })
            }

            fn finish(&self) -> u64 {
                self.0
            }
        }
    }

    // An allocation free HashMap.
    pub(super) struct NoAllocHashMap<const N: usize, K, V> {
        buf: [(K, V); N],
    }

    impl<const N: usize, K, V> NoAllocHashMap<N, K, V>
    where
        K: Hash,
    {
        /// Return the underlying buf to the caller.
        pub fn buf<'a>(&'a self) -> &'a [(K, V); N] {
            &self.buf
        }

        /// Create a new NoAllocHashMap given a function to
        /// fill the underlying buffer.
        pub fn new(fill: impl Fn() -> (K, V)) -> Self {
            Self {
                buf: core::array::from_fn(|_| fill()),
            }
        }

        /// TODO: There's no hasher within core...
        pub fn insert(&mut self, k: K, v: V) {
            todo!();
        }

        pub fn get(&mut self, k: &K) -> Option<&V> {
            todo!()
        }
    }
}

pub trait CombRule<S: SetOperations, T> {
    fn comb_q<const D: usize>(bba: &[&[(S, f32)]; D], q: &S) -> T;
    // TODO: We need 'const generic exprs' in stable to avoid the N2 constraint...
    /// Combine a set of BBAs where we initially compute an approximation, and then after each combination
    /// `m1 comb m2` we compute an approximation.
    fn comb<const N: usize, const N2: usize, A>(bba: &[&[(S, T)]], approx_scheme: A) -> [(S, T); N]
    where
        A: Fn(&[(S, T)]) -> [(S, T); N];
}

pub struct Dempster<S, T>(PhantomData<S>, PhantomData<T>);

impl<S> Dempster<S, f32> where S: SetOperations {}

impl<S> CombRule<S, f32> for Dempster<S, f32>
where
    S: SetOperations + core::hash::Hash + Copy, // TODO: Get rid of the `Copy`.
{
    fn comb_q<const D: usize>(bba: &[&[(S, f32)]; D], q: &S) -> f32 {
        todo!();
    }

    fn comb<const N: usize, const N2: usize, A>(bba: &[&[(S, f32)]], scheme: A) -> [(S, f32); N]
    where
        A: Fn(&[(S, f32)]) -> [(S, f32); N],
    {
        // TODO: See the comment within the trait about 'const generic exprs'. There's `N * N`
        // intersections to compute between each subset after we compute the initial approximation
        // (&[(S, f32)] -> [(S, f32); N]); these have to be placed on the stack...
        // Below function is effectively an unitialized arr (TODO: Maybe we should use `MaybeUninit`?).
        assert!(N2 == N * N);
        fn build_arr<S: SetOperations, const Z: usize>() -> [(S, f32); Z] {
            core::array::from_fn(|_| (S::empty(), 0.0f32))
        }

        bba.iter()
            .map(|e| scheme(e)) // Compute the initial approximation.
            .fold(build_arr::<S, N>(), |acc, e| {
                let mut map = hm::NoAllocHashMap::<N2, S, f32>::new(|| (S::empty(), 0.0f32));
                for (acc_i, e_i) in acc.iter().flat_map(|x1| e.iter().map(move |x2| (x1, x2))) {
                    // B \cap C = m1(B) * m2(C)
                    // map.insert(acc_i.0.intersection(&e_i.0), acc_i.1 * e_i.1);
                }

                // Compute the conflict \frac{1}{1-K}.
                let conflict = 1f32 / (1f32 - map.get(&S::empty()).unwrap_or(&1.0f32));
                let mut tmp = build_arr::<S, N2>();
                for (tmp_i, elem_i) in tmp.iter_mut().zip(map.buf()) {
                    *tmp_i = (elem_i.0, elem_i.1 * conflict);
                }

                scheme(&tmp)
            })
    }
}

pub fn comb_dempster_q<const N: usize, S>(bba_s: [&[(S, f32)]; N], q: S) -> f32
where
    S: Copy + crate::set::SetOperations,
{
    // TODO: Does this first `map` "complete", placing a massive value onto the
    // stack? The below `fold` can be computed for each iteration of the `map`.
    let (k, m) = product::CartesianProduct::new(bba_s)
        .map(|item| {
            item.into_iter()
                .reduce(|(acc_s, acc_m), (set, mass)| (acc_s.intersection(&set), acc_m * mass))
                .unwrap()
        })
        .fold((0.0f32, 0.0f32), |(acc_k, acc_m), (set, mass)| {
            // K = Sum{A = \empty} m; M = Sum{A = Q} m.
            match (set.is_empty(), set.is_subset(&q) && q.is_subset(&set)) {
                (true, false) => (acc_k + mass, acc_m),
                (false, true) => (acc_k, acc_m + mass),
                (_, _) => (acc_k, acc_m),
            }
        });

    (1.0 / (1.0 - k)) * m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comb_dempster_q() {
        // https://en.wikipedia.org/wiki/Dempster%E2%80%93Shafer_theory#Example_producing_correct_results_in_case_of_high_conflict
        const FILM_X: usize = 0b001;
        const FILM_Y: usize = 0b010;
        const FILM_Z: usize = 0b100;

        const FILMS_HIGH_CONFLICT: [&[(usize, f32)]; 2] = [
            &[(FILM_X, 0.99f32), (FILM_Y, 0.01f32)],
            &[(FILM_Z, 0.99f32), (FILM_Y, 0.01f32)],
        ];

        // TODO: Determine what this epsilon should be.
        let eps = 0.001f32;

        assert!((comb_dempster_q(FILMS_HIGH_CONFLICT, FILM_Y) - 1.0f32).abs() < eps);
        assert!(comb_dempster_q(FILMS_HIGH_CONFLICT, FILM_X) < eps);
        assert!(comb_dempster_q(FILMS_HIGH_CONFLICT, FILM_Z) < eps);
    }
}
