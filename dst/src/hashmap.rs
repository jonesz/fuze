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
        CGHashMap { buf }
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
