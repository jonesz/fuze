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
        buf: [(T, T); N],
    }

    impl<const N: usize, T> Interval<N, T> {
        pub fn build(buf: [(T, T); N]) -> Self
        where
            T: PartialOrd,
        {
            assert!(buf.iter().all(|(lhs, rhs)| lhs <= rhs));
            Self { buf }
        }

        fn is_subset(&self, rhs: &Self) -> bool
        where
            T: PartialOrd,
        {
            let f = |l: &(T, T), r: &(T, T)| -> bool { l.0 >= r.0 && l.1 <= r.1 };
            self.buf.iter().zip(rhs.buf.iter()).all(|(l, r)| f(l, r))
        }

        fn op<F>(lhs: &Self, rhs: &Self, f: F) -> Self
        where
            F: Fn(&(T, T), &(T, T)) -> (T, T),
            T: Default + Copy,
        {
            let mut x: [(T, T); N] = [(T::default(), T::default()); N];
            x.iter_mut()
                .zip(lhs.buf.iter().zip(rhs.buf.iter()))
                .for_each(|(mem, (l, r))| *mem = f(l, r));
            return Self { buf: x };
        }

        fn cap(&self, rhs: &Self) -> Self
        where
            T: Ord + Default + Copy,
        {
            let f = |l: &(T, T), r: &(T, T)| -> (T, T) { (T::max(l.0, r.0), T::min(l.1, r.1)) };
            Self::op(self, rhs, f)
        }

        fn cup(&self, rhs: &Self) -> Self
        where
            T: Ord + Default + Copy,
        {
            let f = |l: &(T, T), r: &(T, T)| -> (T, T) { (T::min(l.0, r.0), T::max(l.1, r.1)) };
            Self::op(self, rhs, f)
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
