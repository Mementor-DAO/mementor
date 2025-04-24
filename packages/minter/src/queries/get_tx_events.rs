use std::collections::BTreeMap;
use icrc_ledger_types::{icrc::generic_value::Value, icrc1::account::DEFAULT_SUBACCOUNT};
use crate::{
    storage::transaction::TransactionStorage, 
    types::transaction::{MintReason, TransactionOp}
};

#[ic_cdk::query]
async fn get_tx_events(
    offset: u32,
    size: u32
) -> Result<(Vec<BTreeMap<String, Value>>, u32), String> {

    let txs: Vec<_> = TransactionStorage::slice(offset as _, size as _).iter()
        .map(|(id, tx)| {
            BTreeMap::from([
                ("id".to_string(), Value::Text(hex::encode(id.inner))),
                ("version".to_string(), Value::Nat64(tx.version.into())),
                ("timestamp".to_string(), Value::Nat64(tx.timestamp.into())),
                match &tx.op {
                    TransactionOp::Mint { .. } => {
                        ("type".to_string(), Value::Text("Mint".to_string()))
                    }
                },
                match &tx.op {
                    TransactionOp::Mint { to, .. } => {
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
                        )))
                    }
                },
                match &tx.op {
                    TransactionOp::Mint { amount, .. } => {
                        ("amount".to_string(), Value::Nat((*amount).into()))
                    }
                },
                match &tx.op {
                    TransactionOp::Mint { reason, .. } => {
                        ("reason".to_string(), match reason {
                            MintReason::TeamFee => Value::Text("team fee".to_string()),
                            MintReason::TreasuryFee => Value::Text("treasury fee".to_string()),
                            MintReason::NftMinter(items) => Value::Text(
                                format!(
                                    "nft minter: {}{}", 
                                    items.iter()
                                        .take(10)
                                        .map(|i| i.to_string())
                                        .collect::<Vec<_>>()
                                        .join(",")
                                    ,
                                    if items.len() > 10 {"..."} else {""}
                                )
                            ),
                            MintReason::RaffleWinner => Value::Text("raffle winner".to_string()),
                        })
                    }
                }
            ])
        })
        .collect();
    
    let total_txs = TransactionStorage::size();

    Ok((txs, total_txs))
}

