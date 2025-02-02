//! Core DST operations: `bel` and `pl` corresponding to the
//! calculation of belief and plausabilty respectivey.
use crate::{approx::Approximation, comb::CombRule, set::Set};
use core::{iter::Sum, ops::Sub};

/// Compute the belief of `Q` given a BBA.
pub fn bel<'a, S, T>(bba: impl IntoIterator<Item = &'a (S, T)>, q: &S) -> T
where
    S: Set + 'a,
    T: Sum<&'a T> + 'a,
{
    bba.into_iter() // \sum_{P \subset_eq Q} m(P)
        .filter_map(|(p, m)| if p.is_subset(q) { Some(m) } else { None })
        .sum()
}

/// Compute the plausability of 'Q' given a BBA.
pub fn pl<'a, S, T>(bba: impl IntoIterator<Item = &'a (S, T)>, q: &S) -> T
where
    S: Set + 'a,
    T: Sum<&'a T> + From<u8> + Sub<Output = T> + 'a,
{
    T::from(1u8) - bel(bba, &q.not())
}

/// Combine a set of BBAs with an approximation and combination rule.
pub fn comb_approx<'a, const N: usize, S, T, A, C>(
    // TODO: The above takes a reference, but this one consumes. `Approximation`
    // in some sense needs to consume; we could take a reference and immediately copy
    // it?
    bba: impl IntoIterator<Item = impl IntoIterator<Item = (S, T)>>,
) -> [(S, T); N]
where
    S: Set + 'a,
    T: From<u8> + 'a,
    A: Approximation<S, T>,
    C: CombRule<S, T>,
{
    let mut iter = bba
        .into_iter()
        .map(|e| A::approx(e)) // Compute the initial approximation.
        .reduce(|acc: [Option<(S, T)>; N], e| A::approx(C::comb(&acc, &e))) // Proceed to combine them, then approximate again.
        .expect("Called combination on an empty BBA?")
        .into_iter()
        .flatten();

    core::array::from_fn(|_| iter.next().unwrap_or((S::EMPTY, 0u8.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f32 = 0.001;

    pub(super) mod traffic_light {
        // A test case corresponding to the colors on a traffic
        // light; found on Wikipedia.
        pub const RED: usize = 0b100;
        pub const YELLOW: usize = 0b010;
        pub const GREEN: usize = 0b001;

        pub const TRAFFIC_BBA: &[(usize, f32)] = &[
            (RED, 0.35f32),
            (YELLOW, 0.25f32),
            (GREEN, 0.15f32),
            (RED | YELLOW, 0.06f32),
            (RED | GREEN, 0.05f32),
            (YELLOW | GREEN, 0.04f32),
            (RED | YELLOW | GREEN, 0.1f32),
        ];
    }

    #[test]
    fn test_bel() {
        use traffic_light::*;
        assert_eq!(bel(TRAFFIC_BBA, &RED), 0.35f32);
        assert_eq!(bel(TRAFFIC_BBA, &YELLOW), 0.25f32);
        assert_eq!(bel(TRAFFIC_BBA, &GREEN), 0.15f32);
        assert_eq!(bel(TRAFFIC_BBA, &(RED | YELLOW)), 0.66f32);
        assert_eq!(bel(TRAFFIC_BBA, &(RED | GREEN)), 0.55f32);
        assert_eq!(bel(TRAFFIC_BBA, &(YELLOW | GREEN)), 0.44f32);
        assert_eq!(bel(TRAFFIC_BBA, &(RED | YELLOW | GREEN)), 1.0f32);
    }

    #[test]
    fn test_pl() {
        use traffic_light::*;
        assert!((pl(TRAFFIC_BBA, &RED) - 0.56f32).abs() < TOL);
        assert!((pl(TRAFFIC_BBA, &YELLOW) - 0.45f32).abs() < TOL);
        assert!((pl(TRAFFIC_BBA, &GREEN) - 0.34f32).abs() < TOL);
        assert!((pl(TRAFFIC_BBA, &(RED | YELLOW)) - 0.85f32).abs() < TOL);
        assert!((pl(TRAFFIC_BBA, &(RED | GREEN)) - 0.75f32).abs() < TOL);
        assert!((pl(TRAFFIC_BBA, &(YELLOW | GREEN)) - 0.65f32).abs() < TOL);
        assert!((pl(TRAFFIC_BBA, &(RED | YELLOW | GREEN)) - 1.0f32).abs() < TOL);
    }
}
