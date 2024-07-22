//! Combination rules for Dempster-Shafer Theory.
use crate::container::em::SummationEM;
use crate::set::Set;

pub trait CombRule<S: Set, T> {
    fn comb<const N: usize>(a: &[(S, T); N], b: &[(S, T); N]) -> impl Iterator<Item = (S, T)>;
}

pub struct Dempster();

impl<S> CombRule<S, f32> for Dempster
where
    S: Set,
{
    fn comb<const N: usize>(
        a: &[(S, f32); N],
        b: &[(S, f32); N],
    ) -> impl Iterator<Item = (S, f32)> {
        let mut conflict = 0.0f32; // K.
        let mut map: SummationEM<N, S, f32> = SummationEM::default();

        for (j, k) in a.iter().flat_map(|j| b.iter().map(move |k| (j, k))) {
            let j_cap_k = S::cap(&j.0, &k.0);
            let j_mul_k = j.1 * k.1;

            if j_cap_k == S::EMPTY {
                conflict += j_mul_k;
            } else {
                map.insert(j_cap_k, j_mul_k);
            }
        }

        map.scale(1f32 / (1f32 - conflict));
        map.consume()
    }
}
