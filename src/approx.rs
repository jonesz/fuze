mod magnitude {

    pub struct Magnitude<const N: usize, T: Copy> {
        buf: [(T); N],
    }

    impl<const N: usize, T: Copy> Magnitude<N, T> {
        fn index(&self, x: &T) -> Option<usize> {
            todo!("Calculate the index.")
        }

        pub fn insert(&mut self, x: T) {
            if let Some(i) = self.index(&x) {
                for j in 0..i {
                    *self.buf.get_mut(j).unwrap() = *self.buf.get(j + 1).unwrap();
                }
                *self.buf.get_mut(i).unwrap() = x;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}

/// Perform `summarize` resulting in `N` entires within the BBA.
pub fn summarize<const N: usize, S, T>(bba: &[(S, T)]) -> [(S, T); N]
where
    T: core::cmp::Ord, // TODO: Ord vs PartialOrd?
{
    // Check for the degenerate case where we have less than N elements.
    if bba.len() <= N {
        todo!("Capture the degenerate case.");
    };

    // Smallest to largest.
    let mut largest_masses: [(S, T); N] = todo!("Load the first three values of the bba.");

    let mut insert_largest_masses = |(a_s, a_m): &(S, T)| {
        // Given the above set and mass, determine whether it is large enough to be included.
        let i = for (j, (b_s, b_m)) in largest_masses.iter().enumerate() {
            if j == (N - 1) {
                // If this is the final iteration, we must return a value.
                if b_m > a_m {
                    // Found the largest value.
                    todo!("Figure out these indices.");
                } else {
                    // Found the second largest value.
                    todo!("Figure out these indices.");
                }
            } else {
                if b_m <= a_m {
                    todo!("Figure out these indices.");
                }
            }
        };

        todo!("Given this index, act!");
    };

    // Find the N-1 largest masses.
    for x in bba.iter() {
        insert_largest_masses(x);
    }

    todo!("Create the summary.");
}
