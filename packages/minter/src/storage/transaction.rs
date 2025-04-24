use std::cell::RefCell;
use ic_stable_structures::{BTreeMap, Vec};
use crate::{
    memory::{get_txs_memory, get_txs_ord_memory, Memory}, 
    types::transaction::{Transaction, TxId}
};

pub struct TransactionStorage;

thread_local! {
    static TXS: RefCell<BTreeMap<TxId, Transaction, Memory>> = RefCell::new(
        BTreeMap::init(
            get_txs_memory()
        )
    );
    static TXS_ORD: RefCell<Vec<TxId, Memory>> = RefCell::new(
        Vec::init(
            get_txs_ord_memory()
        ).unwrap()
    );
}

impl TransactionStorage {
    pub fn save(
        id: TxId,
        transaction: Transaction
    ) {
        TXS.with_borrow_mut(|txs| {
            if txs.insert(id.clone(), transaction).is_none() {
                TXS_ORD.with_borrow_mut(|tx_ids| {
                    let _ = tx_ids.push(&id);
                });
            }
        });
    }

    #[allow(unused)]
    pub fn load(
        id: &TxId
    ) -> Option<Transaction> {
        TXS.with_borrow(|txs| {
            txs.get(&id)
        })
    }

    pub fn slice(
        offset: usize,
        size: usize
    ) -> std::vec::Vec<(TxId, Transaction)> {
        let mut slice = vec![];

        TXS_ORD.with_borrow(|tx_ids| {
            TXS.with_borrow(|txs| {
                for tx_id in tx_ids.iter()
                    .skip(offset)
                    .take(size) {
                    let tx = txs.get(&tx_id).unwrap();
                    slice.push((tx_id, tx));
                }
            })
        });

        slice
    }

    pub fn size(
    ) -> u32 {
        TXS_ORD.with_borrow(|txs|
            txs.len() as u32
        )
    }
}