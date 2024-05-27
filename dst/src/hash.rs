//! Hash functions.
use core::hash::Hasher;

#[derive(Debug)]
struct FNV1A(u64);

impl Default for FNV1A {
    fn default() -> Self {
        Self(Self::FNV_OFFSET_BASIS)
    }
}

impl FNV1A {
    // FNV-1a; taken from Wikipedia.
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
}

impl Hasher for FNV1A {
    fn write(&mut self, bytes: &[u8]) {
        self.0 = bytes.iter().fold(self.0, |state, byte| {
            let mut tmp = state;
            tmp ^= Into::<u64>::into(*byte);
            tmp = tmp.overflowing_mul(Self::FNV_PRIME).0;
            tmp
        })
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::hash::Hash;

    #[test]
    fn test_fnv1a() {
        let mut state = FNV1A::default();
        "Hello, World".hash(&mut state);
        assert_ne!(state.finish(), 0u64);
    }
}
