use crate::set::SetOperations;
use core::hash::Hash;
use core::marker::PhantomData;

// TODO: Some sort of ARX cipher might end up being faster? Would remove the mul operation.
mod hash {
    use core::hash::Hasher;

    // FNV-1a; taken from Wikipedia.
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    #[derive(Debug)]
    pub(super) struct FNV1A(u64);

    impl Default for FNV1A {
        fn default() -> Self {
            Self(FNV_OFFSET_BASIS)
        }
    }

    impl Hasher for FNV1A {
        fn write(&mut self, bytes: &[u8]) {
            self.0 = bytes.iter().fold(self.0, |state, byte| {
                let mut tmp = state;
                tmp ^= Into::<u64>::into(*byte);
                tmp = tmp.overflowing_mul(FNV_PRIME).0;
                tmp
            })
        }

        fn finish(&self) -> u64 {
            self.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use core::hash::Hash;

        #[test]
        fn test_fnv1a() {
            let mut state = FNV1A::default();
            "Hello, World".hash(&mut state);
            assert_ne!(state.finish(), 0u64);
        }
    }
}

mod hashmap {
    use super::hash::FNV1A as HashFN;
    use core::hash::{Hash, Hasher};
    use core::ops::{Add, Mul};

    /// An allocation free HashMap with a few useful features suited for our usage.
    pub(super) struct NoAllocHashMap<const N: usize, K, V> {
        // At each iteration we compute an intersection which isn't used elsewhere;
        // just have this buffer own the KV pairs.
        buf: [Option<(K, V)>; N],
    }

    impl<const N: usize, K, V> Default for NoAllocHashMap<N, K, V> {
        fn default() -> Self {
            Self {
                buf: core::array::from_fn(|_| None),
            }
        }
    }

    impl<const N: usize, K, V> NoAllocHashMap<N, K, V>
    where
        K: Eq + Hash,
        V: Add<Output = V> + Mul<Output = V> + Copy,
    {
        // Determine the offset to start at within our linear probe.
        fn compute_offset(key: &K) -> usize {
            let mut state = HashFN::default();
            key.hash(&mut state);
            // 'as'/truncation is fine -- it's a computed offset.
            (state.finish() as usize) % N
        }

        /// Scale the entire Value portion of the HashMap by some value.
        pub fn scale(&mut self, scale: V) {
            self.buf
                .iter_mut()
                .flatten()
                .for_each(|(_, v)| *v = *v * scale);
        }

        /// Return the underlying buf to the caller.
        pub fn buf(&self) -> &[Option<(K, V)>; N] {
            &self.buf
        }

        /// Attempt to insert a value into the HashMap; if it exists, add
        /// the mass value `v` to the existing store.
        pub fn insert(&mut self, k: K, v: V) {
            let offset = Self::compute_offset(&k);

            // TODO: It would be nice to be able to do this with iterators, but
            // we need to start from a specific offset to approach O(1);
            // perhaps it's just flatout faster to use a straight arr and just
            // bruteforce search -- we'd get to use the iterator primitives!
            // Each intersection is unlikely to produce `N` values...

            for i in 0..N {
                let idx = (offset + i) % N;
                if let Some(mem) = self.buf.get_mut(idx) {
                    if let Some(buf_i) = mem.as_mut() {
                        // A value has been stored here; check if this is where the
                        // insertion belongs via key equality.
                        let (buf_k, buf_m) = buf_i;
                        if buf_k == &k {
                            *buf_m = *buf_m + v;
                            return;
                        } else {
                            continue;
                        }
                    } else {
                        // This index is a `None`, so we store the insertion at
                        // the first available value.
                        *mem = Some((k, v));
                        return;
                    }
                }
                unreachable!("Attempted to access an out-of-bounds index.");
            }
            unreachable!("Table was filled; `N` parameter likely wrong.");
        }

        /// Return the value for a key with the table.
        pub fn get(&mut self, k: &K) -> Option<&V> {
            let offset = Self::compute_offset(k);
            for i in 0..N {
                let idx = (offset + i) % N;
                if let Some(buf_i) = self.buf.get(idx) {
                    if let Some((buf_k, buf_m)) = buf_i.as_ref() {
                        if buf_k == k {
                            return Some(buf_m);
                        } else {
                            continue;
                        }
                    } else {
                        return None;
                    }
                } else {
                    unreachable!("Attempted to access an out-of-bounds index.");
                }
            }
            None
        }
    }

    mod tests {
        use super::*;

        const N: usize = 16;

        fn default_table() -> NoAllocHashMap<N, &'static str, f32> {
            let mut hm = NoAllocHashMap::default();

            hm.insert("Hello", 0.0f32);
            hm.insert("World", 1.0f32);

            hm
        }

        #[test]
        fn test_insert_and_get() {
            let mut hm = default_table();

            assert!(hm.get(&"Hello").is_some_and(|x| *x == 0.0f32));
            hm.insert("Hello", 0.1f32);
            assert!(hm.get(&"Hello").is_some_and(|x| *x == 0.1f32));
            assert!(hm.get(&"World").is_some_and(|x| *x == 1.0f32));

            hm.insert("!", 1995.0f32);
            assert!(hm.get(&"!").is_some_and(|x| *x == 1995.0f32));
        }

        #[test]
        fn test_scale() {
            let mut hm = default_table();

            hm.scale(5.0f32);
            assert!(hm.get(&"Hello").is_some_and(|x| *x == 0.0f32));
            assert!(hm.get(&"World").is_some_and(|x| *x == 5.0f32));
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
    S: SetOperations + Hash + Copy, // TODO: Get rid of the `Copy`.
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
                let mut map = hashmap::NoAllocHashMap::<N2, S, f32>::default();
                for (acc_i, e_i) in acc.iter().flat_map(|x1| e.iter().map(move |x2| (x1, x2))) {
                    // B \cap C = m1(B) * m2(C)
                    map.insert(acc_i.0.intersection(&e_i.0), acc_i.1 * e_i.1);
                }

                // Compute the conflict \frac{1}{1-K} and then scale the arr..
                let conflict = 1f32 / (1f32 - map.get(&S::empty()).unwrap_or(&0.0f32));
                map.scale(conflict);

                // scheme(map.buf().iter().flatten());
                todo!();
            })
    }
}

pub fn comb_dempster_q<const N: usize, S>(bba_s: [&[(S, f32)]; N], q: S) -> f32
where
    S: Copy + crate::set::SetOperations,
{
    todo!();
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
