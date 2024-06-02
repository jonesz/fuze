pub trait SetOperations: Eq {
    /// Return whether this is a subset of RHS.
    fn is_subset(&self, rhs: &Self) -> bool;
    /// Return whether this is the empty set.
    fn is_empty(&self) -> bool;
    /// Return the intersection between this and rhs.
    fn intersection(&self, rhs: &Self) -> Self;
    /// Return the empty set.
    fn empty() -> Self;
    fn not(&self) -> Self;
}

impl SetOperations for usize {
    fn is_subset(&self, rhs: &Self) -> bool {
        *self & rhs == *self
    }

    fn is_empty(&self) -> bool {
        *self == 0usize
    }

    fn intersection(&self, rhs: &Self) -> Self {
        self & rhs
    }

    /// Return the empty set.
    fn empty() -> Self {
        0usize
    }

    fn not(&self) -> Self {
        !self
    }
}

mod interval {
    use super::SetOperations;
    use core::cmp::Ord;

    #[derive(Debug)]
    pub struct Interval<const N: usize, T> {
        // Consider the case where we compute the intersection and a certain dimension
        // is disjoint; we signify this by wrapping each dimension in an `Option`.
        buf: [Option<(T, T)>; N],
    }

    impl<const N: usize, T> Interval<N, T> {
        pub fn build(buf: [Option<(T, T)>; N]) -> Self
        where
            T: PartialOrd,
        {
            assert!(buf.iter().flatten().all(|(lhs, rhs)| lhs <= rhs));
            Self { buf }
        }

        fn is_subset(lhs: &Self, rhs: &Self) -> bool
        where
            T: PartialOrd,
        {
            // ex: `[-1, 5] \subset [-2, 6]`
            let f = |l: &(T, T), r: &(T, T)| -> bool { l.0 >= r.0 && l.1 <= r.1 };
            lhs.buf.iter().zip(rhs.buf.iter()).all(|(l, r)| {
                (l.is_none() && r.is_none()) // Either the dimensions are both `None`...
                    || l.as_ref()
                        .zip(r.as_ref()) // Or they're both `Some` with the subset condition holding.
                        .is_some_and(|(l_i, r_i)| f(l_i, r_i))
            })
        }

        fn binop<F>(lhs: &Self, rhs: &Self, f: F) -> Self
        where
            F: Fn(Option<&(T, T)>, Option<&(T, T)>) -> Option<(T, T)>,
        {
            let mut x: [Option<(T, T)>; N] = core::array::from_fn(|_| None);
            x.iter_mut()
                .zip(lhs.buf.iter().zip(rhs.buf.iter()))
                .for_each(|(mem, (l, r))| *mem = f(l.as_ref(), r.as_ref()));
            Self { buf: x }
        }

        fn cap(lhs: &Self, rhs: &Self) -> Self
        where
            T: Ord + Copy,
        {
            let f = |l: Option<&(T, T)>, r: Option<&(T, T)>| -> Option<(T, T)> {
                l.zip(r) // `zip` is effectively an AND, then we have the following condition.
                    .map(|(l, r)| (T::max(l.0, r.0), T::min(l.1, r.1)))
            };

            Self::binop(lhs, rhs, f)
        }

        fn cup(lhs: &Self, rhs: &Self) -> Self
        where
            T: Ord + Copy,
        {
            let f = |l: Option<&(T, T)>, r: Option<&(T, T)>| -> Option<(T, T)> {
                match (l, r) {
                    (None, None) => None,
                    (Some(_), None) => l.copied(),
                    (None, Some(_)) => r.copied(),
                    (Some(m), Some(s)) => Some((T::min(m.0, s.0), T::max(m.1, s.1))),
                }
            };

            Self::binop(lhs, rhs, f)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_interval() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
