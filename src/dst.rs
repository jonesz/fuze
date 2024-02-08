use crate::set::SetOperations;

/// Compute the belief of `A` given a BBA.
pub fn bel<'a, S, T>(bba: &'a [(S, T)], q: &S) -> T
where
    S: SetOperations,
    T: core::iter::Sum<&'a T> + 'a,
{
    bba.iter()
        .filter(|(p, _)| p.is_subset(q))
        .map(|(_, mass)| mass)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
}
