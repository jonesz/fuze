// TODO: Perhaps this should be named something like CGHashSummation?
// TODO: What does CG even stand for?
pub struct CGHashMap<const N: usize, K, V> {
    // TODO: We need `generic_const_expr` to compute `N * N`.
    // buf: [Option<(K, V)>; N * N],
    // ... but a [[...; N]; N] dimension arr is potentially a way to avoid this.
    buf: [[Option<(K, V)>; N]; N],
}

impl<const N: usize, K, V> Default for CGHashMap<N, K, V> {
    fn default() -> Self {
        // TODO: If `(K, V)` is `Copy`, this becomes [[None; N]; N] which is much,
        // much nicer. There's also opportunities for `MaybeUninit`? This buffer should
        // just be zeroed (if `None` is just zeroed.)
        let buf: [[Option<(K, V)>; N]; N] =
            core::array::from_fn(|_| core::array::from_fn(|_| None));
        Self { buf }
    }
}

impl<const N: usize, K, V> CGHashMap<N, K, V>
where
    K: PartialEq,
    V: core::ops::AddAssign + core::ops::MulAssign + Copy,
{
    // This 'insert' operation is a summation.
    pub fn insert(&mut self, k: K, v: V) {
        let mem = self
            .buf
            .iter_mut()
            .flatten()
            // We're looking for a `None` or the index where the keys match.
            .find(|z| !z.as_ref().is_some_and(|y| y.0 != k))
            .expect("Should have found an index to place this KV in; is N correct?.");

        if let Some(inner) = mem {
            inner.1 += v;
        } else {
            *mem = Some((k, v));
        }
    }

    pub fn scale(&mut self, s: V) {
        self.buf
            .iter_mut()
            .flatten()
            .flatten()
            // TODO: The `*= s` triggers a `Copy` of `s`; what's the
            // perf impact? We could utilize a reference, but it makes the
            // trait bounds uglier.
            .for_each(|x| x.1 *= s);
    }

    /// Return an iterator over the underlying buffer.
    pub fn consume(self) -> impl Iterator<Item = (K, V)> {
        // [[...; N]; N] -> [...; N * N] alongside dumping all `None` options.
        self.buf.into_iter().flatten().flatten()
    }
}

pub struct PriorityQueue<const N: usize, T> {
    buf: [Option<T>; N],
}

impl<const N: usize, T> Default for PriorityQueue<N, T> {
    fn default() -> Self {
        Self {
            buf: core::array::from_fn(|_| None),
        }
    }
}

impl<const N: usize, T> PriorityQueue<N, T> {
    const LEAF_IDX: usize = usize::ilog2(N) as usize;

    /// Compute the children of the passed index with the array representation of a heap.
    const fn children(x: usize) -> (usize, usize) {
        (x * 2 + 1, x * 2 + 2)
    }

    /// Compute the parent of a child.
    const fn parent(x: usize) -> usize {
        todo!()
    }

    fn heap_upkeep(&mut self, idx: usize) {
        todo!();
    }

    // TODO: `impl Fn(&T) -> R`: this constrains the API to things that can `Copy`.
    pub fn insert_by_key<R: PartialOrd>(&mut self, f: impl Fn(&T) -> R, v: T) -> Option<T> {
        // If there's a `None`, attempt to find it and replace it.
        let (idx, r) =
            if let Some((idx, mem)) = self.buf.iter_mut().enumerate().find(|(_, x)| x.is_none()) {
                (idx, mem.replace(v))
            } else {
                // Otherwise, find the minimum value then replace it.
                let (idx, mem) = self.buf[Self::LEAF_IDX..]
                    .iter_mut()
                    .enumerate()
                    .reduce(|(a, min), (b, e)| {
                        if f(min.as_ref().unwrap()) > f(e.as_ref().unwrap()) {
                            (b, e)
                        } else {
                            (a, min)
                        }
                    })
                    .unwrap();
                (idx, mem.replace(v))
            };

        self.heap_upkeep(idx);
        r
    }

    pub fn consume(self) -> impl Iterator<Item = T> {
        self.buf.into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type PQ = PriorityQueue<16, usize>;

    #[test]
    fn test_children() {
        assert_eq!(PQ::children(0), (1, 2));
        assert_eq!(PQ::children(1), (3, 4));
        assert_eq!(PQ::children(2), (5, 6));
        assert_eq!(PQ::children(3), (7, 8));
    }
}
