use candid::Principal;
use ic_cdk::api::management_canister::main::{
    canister_info, create_canister, install_code, start_canister, 
    CanisterIdRecord, CanisterInfoRequest, CanisterInfoResponse, 
    CanisterInstallMode, CanisterSettings, CreateCanisterArgument, 
    InstallCodeArgument, LogVisibility
};

#[allow(unused)]
pub(crate) async fn get_canister_info(
    canister_id: Principal
) -> Result<CanisterInfoResponse, String> {
    let res = canister_info(CanisterInfoRequest { 
        canister_id, 
        num_requested_changes: None 
    }).await.map_err(|e| e.1)?;

    Ok(res.0)
}

#[allow(unused)]
pub(crate) async fn deploy_canister(
    wasm_module: &[u8],
    arg: Vec<u8>,
    controllers: Option<Vec<Principal>>,
    cycles: u128
) -> Result<Principal, String> {
    
    let res = create_canister(CreateCanisterArgument { 
        settings: Some(CanisterSettings {
            controllers,
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            log_visibility: Some(LogVisibility::Public),
            wasm_memory_limit: None,
        })
    }, cycles).await.map_err(|e| e.1)?;

    let canister_id = res.0.canister_id;

    install_code(InstallCodeArgument { 
        mode: CanisterInstallMode::Install, 
        canister_id, 
        wasm_module: wasm_module.to_vec(), 
        arg 
    }).await.map_err(|e| e.1)?;

    start_canister(CanisterIdRecord { 
        canister_id 
    }).await.map_err(|e| e.1)?;

    Ok(canister_id)
}