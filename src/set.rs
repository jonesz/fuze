pub trait SetOperations {
    /// Return whether this is a subset of RHS.
    fn is_subset(&self, rhs: &Self) -> bool;
    /// Return whether this is the empty set.
    fn is_empty(&self) -> bool;
    /// Return the intersection between this and rhs.
    fn intersection(&self, rhs: &Self) -> Self;
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
}

mod interval {
    use super::SetOperations;
    use core::cmp::Ord;

    pub struct Interval<T: Ord, const N: usize> {
        buf: [Option<(T, T)>; N],
    }

    impl<T: Ord + Copy, const N: usize> Interval<T, N> {
        fn intersect(lhs: &(T, T), rhs: &(T, T)) -> Option<(T, T)> {
            // Check to see if there's any overlap between the intervals.
            if !(lhs.1 > rhs.0 || rhs.1 > lhs.1) {
                return None;
            }

            todo!("Left unimplemented.");
        }
    }

    impl<T: Ord + Copy, const N: usize> SetOperations for Interval<T, N> {
        /// Return whether this is a subset of RHS.
        fn is_subset(&self, rhs: &Self) -> bool {
            self.buf
                .iter()
                .zip(rhs.buf.iter())
                .all(|(left, right)| match (left, right) {
                    (None, None) => true,
                    // Example: [1, 4] \subset [0, 5]
                    (Some(l), Some(r)) => l.0 >= r.0 && l.1 <= r.1,
                    _ => false,
                })
        }

        /// Return whether this is the empty set.
        fn is_empty(&self) -> bool {
            // The empty set should have all None (disjoint) values.
            self.buf.iter().all(|x| x.is_none())
        }

        /// Return the intersection between this and rhs.
        fn intersection(&self, rhs: &Self) -> Self {
            let mut buf = [None; N];
            for (idx, mem) in buf.iter_mut().enumerate() {
                let (left, right) = (self.buf.get(idx).unwrap(), rhs.buf.get(idx).unwrap());
                match (left, right) {
                    (Some(l), Some(r)) => todo!(),
                    _ => *mem = None,
                }
            }
            Self { buf }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_intersect_i32() {
            // Intervals are disjoint.
            let (a, b) = ((-1i32, 0i32), (1i32, 2i32));
            assert!(Interval::<i32, 1>::intersect(&a, &b).is_none());
            // Interval a is contained within interval b.
            let (a, b) = ((-1i32, 1i32), (-2i32, 2i32));
            assert!(Interval::<i32, 1>::intersect(&a, &b).is_some_and(|c| c == a));
            // Interval b is contained within interval a.
            let (a, b) = ((-2i32, 2i32), (-1i32, 1i32));
            assert!(Interval::<i32, 1>::intersect(&a, &b).is_some_and(|c| c == b));
            // Interval a overlaps with interval b on the left side.
            let (a, b) = ((-2i32, 1i32), (0i32, 2i32));
            assert!(Interval::<i32, 1>::intersect(&a, &b).is_some_and(|c| c == (0i32, 1i32)));
            // Interval b overlaps with interval a on the left side.
            let (a, b) = ((-2i32, 1i32), (-3i32, 0i32));
            assert!(Interval::<i32, 1>::intersect(&a, &b).is_some_and(|c| c == (-2i32, 0i32)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
