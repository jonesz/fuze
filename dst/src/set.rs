/// A mathematical Set, but specifically those operations needed for a DST impl.
pub trait Set: PartialEq {
    /// Compute whether the LHS is a subset of the RHS.
    fn is_subset(&self, rhs: &Self) -> bool;
    /// Compute the intersection between the LHS and RHS.
    fn cap(lhs: &Self, rhs: &Self) -> Self;
    /// Compute the union between the LHS and RHS.
    fn cup(lhs: &Self, rhs: &Self) -> Self;
    /// Compute the NOT of some Set.
    fn not(&self) -> Self;

    /// A representation of the Empty Set.
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

impl<const N: usize> Set for [u8; N] {
    fn is_subset(&self, rhs: &Self) -> bool {
        self.iter().zip(rhs).all(|(l, r)| l & r == *l)
    }

    fn cap(lhs: &Self, rhs: &Self) -> Self {
        let mut buf = Self::EMPTY;
        buf.iter_mut()
            .zip(lhs.iter().zip(rhs))
            .for_each(|(x, (l, r))| *x = l & r);
        buf
    }

    fn cup(lhs: &Self, rhs: &Self) -> Self {
        let mut buf = Self::EMPTY;
        buf.iter_mut()
            .zip(lhs.iter().zip(rhs))
            .for_each(|(x, (l, r))| *x = l | r);
        buf
    }

    fn not(&self) -> Self {
        let mut buf = Self::EMPTY;
        buf.iter_mut().zip(self).for_each(|(i, x)| *i = !x);
        buf
    }

    const EMPTY: Self = [0u8; N];
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
                (l.is_none() && r.is_none()) // Either the dimensions are both `None`...
                    || l.as_ref()
                        .zip(r.as_ref()) // Or they're both `Some` with the equivalence condition holding.
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
                        if (l.1 >= r.0) && (r.1 >= l.0) {
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
            // TODO: Consider the case where there's two disjoint intervals --
            // we can't represent their union!
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
        const VALUES: [((i32, i32), (i32, i32)); 4] = [
            ((0, 10), (1, 11)),  // A.0 < B.0 && A.1 < B.1
            ((0, 10), (-1, 11)), // A.0 > B.0 && A.1 < B.1
            ((0, 10), (1, 9)),   // A.0 < B.0 && A.1 > B.1
            ((0, 10), (-1, 9)),  // A.0 > B.0 && A.1 > B.1
        ];

        #[test]
        fn test_interval_is_subset_regular() {
            const EXPECTED: [bool; VALUES.len()] = [false, true, false, false];
            for ((elem_a, elem_b), result) in VALUES.into_iter().zip(EXPECTED) {
                let a = Interval::build([Some(elem_a)]);
                let b = Interval::build([Some(elem_b)]);
                assert_eq!(a.is_subset(&b), result);
            }
        }

        #[test]
        fn test_interval_is_subset_irregular() {
            let b = Interval::build([Some((0, 10))]);

            assert!(Interval::<1, i32>::EMPTY.is_subset(&Interval::EMPTY)); // None \subset None :thumbs_up:.
            assert!(!Interval::EMPTY.is_subset(&b)); // !(None \subset Some) :thumbs_up:.
            assert!(!b.is_subset(&Interval::EMPTY)); // !(Some \subset None) :thumbs_up:.
        }

        #[test]
        fn test_interval_cup() {
            const EXPECTED: [(i32, i32); VALUES.len()] = [(0, 11), (-1, 11), (0, 10), (-1, 10)];
            for ((elem_a, elem_b), result) in VALUES.into_iter().zip(EXPECTED) {
                let a = Interval::build([Some(elem_a)]);
                let b = Interval::build([Some(elem_b)]);
                let c = Interval::build([Some(result)]);
                assert_eq!(Interval::cup(&a, &b), c);
            }
        }

        #[test]
        fn test_interval_cup_disjoint() {
            let a = Interval::build([Some((0, 10))]);
            let b = Interval::build([Some((11, 20))]);
            let c = Interval::cup(&a, &b); // A \cup B = (0, 10) U (11, 20).
            assert_eq!(c, todo!());
        }

        #[test]
        fn test_interval_cup_irregular() {
            let b = Interval::build([Some((0, 10))]);
            assert_eq!(Interval::cup(&Interval::EMPTY, &b), b); // EMPTY \cup B = B.
            assert_eq!(
                Interval::<1, i32>::cup(&Interval::EMPTY, &Interval::EMPTY),
                Interval::EMPTY
            ); // EMPTY \cup EMPTY = EMPTY.
        }

        #[test]
        fn test_interval_cap() {
            const EXPECTED: [(i32, i32); VALUES.len()] = [(1, 10), (0, 10), (1, 9), (0, 9)];
            for ((elem_a, elem_b), result) in VALUES.into_iter().zip(EXPECTED) {
                let a = Interval::build([Some(elem_a)]);
                let b = Interval::build([Some(elem_b)]);
                let c = Interval::build([Some(result)]);
                assert_eq!(Interval::cap(&a, &b), c);
            }
        }

        #[test]
        fn test_interval_cap_disjoint() {
            let a = Interval::build([Some((0, 10))]);
            let b = Interval::build([Some((11, 20))]);
            assert_eq!(Interval::cap(&a, &b), Interval::EMPTY);
        }

        #[test]
        fn test_interval_cap_irregular() {
            let b = Interval::build([Some((0, 10))]);
            assert_eq!(Interval::cap(&Interval::EMPTY, &b), Interval::EMPTY); // EMPTY \cap B = EMPTY.
        }

        #[test]
        fn test_interval_not() {
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
