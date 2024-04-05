/// Example taken from PLG's `1.4 Gentle Start`.
use rand::Rng;
const EXPERT_NUM: usize = 64;

// An environment that always returns `1`.
fn env(t: usize) -> u8 {
    return 1;
}

fn expert<R: Rng>(t: usize, i: usize, rng: &mut R) -> u8 {
    // The first expert is always right.
    if i == 0 {
        1
    } else {
        // Some fuction that isn't constant with range {0, 1}.
        let r: usize = rng.gen();
        if r % i == 0 {
            0
        } else {
            1
        }
    }
}

#[test]
fn gentle_start() {
    let weights = [1u8; EXPERT_NUM];
    let mut rng = rand::thread_rng();

    while (weights.iter().sum::<u8>() != 1) {
        
    }
}
