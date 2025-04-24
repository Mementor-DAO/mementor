use candid::Principal;
use icrc_ledger_types::icrc1::{
    account::{principal_to_subaccount, Account}, 
    transfer::{TransferArg, TransferError}
};

pub struct ICRC1;

#[allow(unused)]
impl ICRC1 {    
    pub fn principals_to_account(
        principal: Principal,
        secondary: Principal
    ) -> Account {
        Account{
            owner: principal,
            subaccount: Some(principal_to_subaccount(secondary)),
        }
    }
    
    pub async fn balance_of(
        canister_id: &Principal,
        account: Account
    ) -> Result<u128, String> {

        let (res, ): (u128, ) = ic_cdk::call(
            canister_id.clone(), 
            "icrc1_balance_of",
            (&account, )
        ).await.map_err(|e| e.1)?;

        Ok(res)
    }

    pub async fn transfer(
        canister_id: &Principal,
        arg: TransferArg
    ) -> Result<u128, String> {

        let (res, ): (Result<u128, TransferError>, ) = ic_cdk::call(
            canister_id.clone(), 
            "icrc1_transfer",
            (&arg, )
        ).await.map_err(|e| e.1)?;

        res.map_err(|e| e.to_string())
    }

    pub async fn minting_account(
        canister_id: &Principal
    ) -> Result<Option<Account>, String> {

        let (res, ): (Option<Account>, ) = ic_cdk::call(
            canister_id.clone(), 
            "icrc1_minting_account",
            ()
        ).await.map_err(|e| e.1)?;

        Ok(res)
    }
}