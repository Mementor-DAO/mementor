use icrc_ledger_types::icrc1::account::Subaccount;
use sha2::{Digest, Sha256};

pub fn str_to_subaccount(
    text: &str
) -> Subaccount {
    const DOMAIN: &[u8] = b"str-id";
    const DOMAIN_LENGTH: [u8; 1] = [6];

    let mut hasher = Sha256::new();
    _ = hasher.update(&DOMAIN_LENGTH);
    _ = hasher.update(&DOMAIN);
    _ = hasher.update(&text.as_bytes());
    hasher.finalize().into()
}