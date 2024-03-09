use crate::product;

pub fn comb_dempster_q<const N: usize, S>(bba_s: [&[(S, f32)]; N], q: S) -> f32
where
    S: Copy + crate::set::SetOperations,
{
    let p = product::CartesianProduct::new(bba_s);

    let mut k = 0.0f32;
    let mut s = 0.0f32;

    for item in p {
        let (set, mass) = item
            .into_iter()
            .reduce(|(acc_s, acc_m), (s, m)| (acc_s.intersection(&s), acc_m * m))
            .unwrap();

        if set.is_empty() {
            k += mass;
        } else {
            if set.is_subset(&q) && q.is_subset(&set) {
                s += mass;
            }
        }
    }

    (1.0 / (1.0 - k)) * s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comb_dempster_q() {}
}
