import { Principal } from "@dfinity/principal";

const isProduction = process.env.DFX_NETWORK === 'ic';

export const config = {
    isProduction,
    IC_GATEWAY: isProduction?
        'https://ic0.app':
        undefined,
    IC_URL: isProduction?
        process.env.VITE_IC_URL:
        process.env.VITE_IC_URL_LOCAL,
    II_URL: isProduction?
        process.env.VITE_II_URL:
        process.env.VITE_II_URL_LOCAL,
    APP_URL: isProduction?
        process.env.VITE_APP_URL:
        process.env.VITE_APP_URL_LOCAL,
    ICP_LEDGER_CANISTER_ID: 
        process.env.VITE_ICP_LEDGER_CANISTER_ID?
            Principal.fromText(process.env.VITE_ICP_LEDGER_CANISTER_ID):
            Principal.anonymous()
}