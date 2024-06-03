/// A mathematical Set, but specifically those operations needed for a DST impl.
pub trait Set: PartialEq {
    /// Compute whether the LHS is a subset of the RHS.
    fn is_subset(&self, rhs: &Self) -> bool;
    /// Compute the intersection between the LHS and RHS.
    fn cap(lhs: &Self, rhs: &Self) -> Self;
    /// Compute the union betweenthe LHS ad RHS.
    fn cup(lhs: &Self, rhs: &Self) -> Self;
    /// Compute the NOT of some Set.
    fn not(&self) -> Self;

    const EMPTY: Self;
}

impl Set for usize {
    fn is_subset(&self, rhs: &Self) -> bool {
        self & rhs == *self
    }

    fn cap(lhs: &Self, rhs: &Self) -> Self {
        lhs & rhs
    }

    fn cup(lhs: &Self, rhs: &Self) -> Self {
        lhs | rhs
    }

    fn not(&self) -> Self {
        !self
    }

    const EMPTY: Self = 0usize;
}

mod interval {
    use super::Set;
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
    }

    impl<const N: usize, T> PartialEq for Interval<N, T>
    where
        T: PartialEq,
    {
        fn eq(&self, rhs: &Self) -> bool {
            self.buf.iter().zip(rhs.buf.iter()).all(|(l, r)| {
                l.as_ref()
                    .zip(r.as_ref())
                    .is_some_and(|(l_i, r_i)| l_i.0 == r_i.0 && l_i.1 == r_i.1)
            })
        }
    }

    impl<const N: usize, T> Set for Interval<N, T>
    where
        T: PartialOrd + Ord + Copy,
    {
        fn is_subset(&self, rhs: &Self) -> bool {
            // ex: `[-1, 5] \subset [-2, 6]`
            let f = |l: &(T, T), r: &(T, T)| -> bool { l.0 >= r.0 && l.1 <= r.1 };
            self.buf.iter().zip(rhs.buf.iter()).all(|(l, r)| {
                (l.is_none() && r.is_none()) // Either the dimensions are both `None`...
                    || l.as_ref()
                        .zip(r.as_ref()) // Or they're both `Some` with the subset condition holding.
                        .is_some_and(|(l_i, r_i)| f(l_i, r_i))
            })
        }

        fn cap(lhs: &Self, rhs: &Self) -> Self {
            let f = |l: Option<&(T, T)>, r: Option<&(T, T)>| -> Option<(T, T)> {
                l.zip(r) // `zip` is effectively an AND...
                    .and_then(|(l, r)| {
                        // And we have a condition that guarantees overlap between intervals.
                        if (l.1 >= r.0) || (r.1 >= l.0) {
                            Some((T::max(l.0, r.0), T::min(l.1, r.1)))
                        } else {
                            None
                        }
                    })
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

        fn not(&self) -> Self {
            // TODO: The NOT of an `Interval` is two `Interval`s; a potential implementation
            // is to utilize some flag to indicate that `subset`, `cup`, etc. should NOT their
            // results?
            unimplemented!();
        }

        const EMPTY: Self = Self { buf: [None; N] };
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_interval() {
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
