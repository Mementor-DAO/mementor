use crypto_bigint::U256;
use sha2::{Digest, Sha256};

pub struct DblHasher {
    sha: Sha256,
}

impl DblHasher {
    pub fn new(
    ) -> Self {
        Self {
            sha: Sha256::new()
        }
    }

    pub fn hash(
        &mut self,
        value: &[u8]
    ) -> U256 {
        self.sha.update(value);
        let mut first = [0u8; 32].into();
        self.sha.finalize_into_reset(&mut first);

        self.sha.update(first);
        let mut second = [0u8; 32].into();
        self.sha.finalize_into_reset(&mut second);

        U256::from_le_slice(&second)
    }
}