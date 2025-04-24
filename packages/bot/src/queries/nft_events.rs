use std::collections::BTreeMap;
use icrc_ledger_types::{icrc::generic_value::Value, icrc1::account::DEFAULT_SUBACCOUNT};
use crate::{
    services::nft, 
    storage::{event::EventStorage, nft::NftStorage}, 
    types::event::Event
};

#[ic_cdk::query]
async fn get_nft_events(
    offset: u32,
    size: u32
) -> Result<(Vec<BTreeMap<String, Value>>, u32), String> {

    let events = EventStorage::slice(offset as _, size as _).iter()
        .map(|e| {
            match e {
                Event::NftMinted { token_id, to, timestamp } => {
                    let logo = get_nft_logo(token_id).or(Some("".to_string())).unwrap();
                    BTreeMap::from([
                        ("type".to_string(), Value::Text("NFT Minted".into())),
                        ("token_id".to_string(), Value::Nat((*token_id).into())),
                        ("to".to_string(), Value::Text(format!(
                            "{}{}",
                            to.owner.to_text(),
                            to.subaccount.map(|s| 
                                if s == *DEFAULT_SUBACCOUNT {
                                    "".to_string()
                                } 
                                else {
                                    format!(".{}", hex::encode(s))
                                }
                            ).or(Some("".to_string())).unwrap()
                        ))),
                        ("timestamp".to_string(), Value::Nat64(*timestamp as _)),
                        ("logo".to_string(), Value::Text(logo)),
                    ])
                },
                Event::NftListed { token_id, value, timestamp } => {
                    let logo = get_nft_logo(token_id).or(Some("".to_string())).unwrap();
                    BTreeMap::from([
                        ("type".to_string(), Value::Text("NFT Listed".into())),
                        ("token_id".to_string(), Value::Nat((*token_id).into())),
                        ("value".to_string(), Value::Nat((*value).into())),
                        ("timestamp".to_string(), Value::Nat64(*timestamp as _)),
                        ("logo".to_string(), Value::Text(logo)),
                    ])
                },
                Event::NftSold { token_id, value, timestamp } => {
                    let logo = get_nft_logo(token_id).or(Some("".to_string())).unwrap();
                    BTreeMap::from([
                        ("type".to_string(), Value::Text("NFT Sold".into())),
                        ("token_id".to_string(), Value::Nat((*token_id).into())),
                        ("value".to_string(), Value::Nat((*value).into())),
                        ("timestamp".to_string(), Value::Nat64(*timestamp as _)),
                        ("logo".to_string(), Value::Text(logo)),
                    ])
                },
            }
        })
        .collect::<Vec<_>>();
    
    let total_events = EventStorage::size();
    
    Ok((events, total_events))
}

fn get_nft_logo(
    token_id: &u128
) -> Option<String> {
    let nft = NftStorage::load(token_id)?;

    let url = nft::read(|s| 
        s.col.url_template.replace("{}", &nft.blob_id.to_string())
    );

    Some(url.to_string())
}