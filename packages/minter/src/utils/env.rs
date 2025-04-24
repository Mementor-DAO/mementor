use sha2::{Digest, Sha256};

pub fn entropy() -> [u8; 32] {
    let mut bytes = Vec::new();

    bytes.extend(ic_cdk::id().as_slice());
    bytes.extend(ic_cdk::caller().as_slice());
    bytes.extend(ic_cdk::api::time().to_ne_bytes());
    bytes.extend(ic_cdk::api::canister_balance().to_ne_bytes());
    bytes.extend(ic_cdk::api::call::arg_data_raw());

    sha256(&bytes)
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}