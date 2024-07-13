//! Core DST operations: `bel` and `pl` corresponding to the
//! calculation of belief and plausabilty respectivey.
use crate::set::Set;
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

pub fn comb_approx<'a, const N: usize, S, T>(
    bba: impl IntoIterator<Item = impl IntoIterator<Item = &'a (S, T)>>,
) -> [(S, T); N] {
    todo!();
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
