use cw_orch::environment::ChainInfoOwned;
// ANCHOR: custom_interface
use cw_orch::{interface, prelude::*};
use crate::contract::CONTRACT_NAME;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
pub const CONTRACT_ID: &str = CONTRACT_NAME;

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg, id = CONTRACT_ID)]
pub struct MulticallContract;

impl<Chain> Uploadable for MulticallContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("multiquery")
            .unwrap()
    }

    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            )
                .with_migrate(crate::contract::migrate),
        )
    }
}
