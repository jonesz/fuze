use crate::set::SetOperations;
use core::iter::Sum;

/// Compute the belief of `A` given a BBA.
pub fn bel<'a, S, T>(bba: &'a [(S, T)], q: &S) -> T
where
    S: SetOperations,
    T: Sum<&'a T> + 'a,
{
    bba.iter()
        .filter(|(p, _)| p.is_subset(q))
        .map(|(_, mass)| mass)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bel() {
        const RED: usize = 0b100;
        const YELLOW: usize = 0b010;
        const GREEN: usize = 0b001;

        let bba = &[
            (RED, 0.35f32),
            (YELLOW, 0.25f32),
            (GREEN, 0.15f32),
            (RED | YELLOW, 0.06f32),
            (RED | GREEN, 0.05f32),
            (YELLOW | GREEN, 0.04f32),
            (RED | YELLOW | GREEN, 0.1f32),
        ];

        assert_eq!(bel(bba, &RED), 0.35f32);
        assert_eq!(bel(bba, &YELLOW), 0.25f32);
        assert_eq!(bel(bba, &GREEN), 0.15f32);
        assert_eq!(bel(bba, &(RED | YELLOW)), 0.66f32);
        assert_eq!(bel(bba, &(RED | GREEN)), 0.55f32);
        assert_eq!(bel(bba, &(YELLOW | GREEN)), 0.44f32);
        assert_eq!(bel(bba, &(RED | YELLOW | GREEN)), 1.0f32);
    }
}
