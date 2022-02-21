/*!
 * xINT NEP-141 Token contract
 *
 */
// use std::iter::Map;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
// Needed by `impl_fungible_token_core` for old Rust.
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
use near_sdk::borsh::maybestd::collections::LinkedList;
// use crate::utils::DURATION_30DAYS_IN_SEC;
use crate::utils::DURATION_1H_IN_SEC;
pub use crate::utils::nano_to_sec;
pub use crate::views::ContractMetadata;

mod xpnx;
mod utils;
mod owner;
mod views;
mod storage_impl;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub ft: FungibleToken,
    pub owner_id: AccountId,
    pub locked_token: AccountId,
    /// deposit reward that does not distribute to locked REF yet
    pub undistributed_reward: Balance,
    /// locked amount
    pub locked_token_amount: Balance,
    /// the previous distribution time in seconds
    pub prev_distribution_time_in_sec: u32,
    /// when would the reward starts to distribute
    pub reward_genesis_time_in_sec: u32,
    pub reward_per_sec: Balance,
    /// current account number in contract
    pub account_number: u64,
    pub account_list: LinkedList<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId, locked_token: ValidAccountId) -> Self {
        let initial_reward_genisis_time = DURATION_1H_IN_SEC + nano_to_sec(env::block_timestamp());
        Contract {
            ft: FungibleToken::new(b"a".to_vec()),
            owner_id: owner_id.into(),
            locked_token: locked_token.into(),
            undistributed_reward: 0,
            locked_token_amount: 0,
            prev_distribution_time_in_sec: initial_reward_genisis_time,
            reward_genesis_time_in_sec: initial_reward_genisis_time,
            reward_per_sec: 0,
            account_number: 0,
            account_list: LinkedList::new(),
        }
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);
const DATA_IMAGE_SVG_PNX_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGNpcmNsZSBjeD0iMjAiIGN5PSIyMCIgcj0iMjAiIGZpbGw9IiMwNzA2MzgiLz48cGF0aCBkPSJNMjguNDU1IDI5LjczMmMtMi41MTQgMi44NTUtNy4wNSA2LjEyLTExLjAxNyA2LjE1My0yLjUxNy0uMzI0LTQuODM3LTEuMTE2LTYuODE1LTIuMjYyIDIuNTAyLjE3NCA1LjUxNS0uMjc3IDguNDI3LTEuNzkgMS4zOTktLjYwNSAzLjYzMy0yLjIzOCAzLjcxOC0yLjI5Ny0uMDY2LjAzNy0yLjQ5IDEuNDgyLTMuOTQ2IDEuOTY1LTQuMzQ0IDEuNzkxLTguNjQ3IDEuMzEyLTEwLjc2Mi4yNzItMi40OTktMi4yMzgtNC4wMy01LjE4LTQuMDYtOC40MS4zMyAzLjEwMyA1Ljg4OCA2LjI1NCAxMS41NCA1LjEyNyA2LjQtMS4yNzcgMTAuNzM0LTcuNjQ1IDE1LjgzNC04LjQxNCAxLjE1OS0uMTc1IDIuNDY2LS4wNyAzLjM2Ny41MzggMS4wNzUuNzI2IDEuMjI3IDEuNTUgMS4yNTUgMi42NTIuMTc5IDYuNjktNi40MzUgMTIuMTc5LTE0LjYzIDEyLjczNCAyLjg4My0xLjQwOSA1LjQzNy0zLjg2NSA3LjA4OS02LjI2OFoiIGZpbGw9InVybCgjYSkiLz48cGF0aCBkPSJNMTEuNTQ1IDEwLjI2OGMyLjUxNC0yLjg1NSA3LjA1LTYuMTIgMTEuMDE3LTYuMTUzIDIuNTE3LjMyMyA0LjgzNyAxLjExNiA2LjgxNSAyLjI2Mi0yLjUwMi0uMTc0LTUuNTE1LjI3Ny04LjQyNyAxLjc5LTEuMzk5LjYwNS0zLjYzMyAyLjIzOC0zLjcxOCAyLjI5Ny4wNjYtLjAzNyAyLjQ5LTEuNDgyIDMuOTQ2LTEuOTY1IDQuMzQ0LTEuNzkxIDguNjQ4LTEuMzEyIDEwLjc2Mi0uMjcyIDIuNDk5IDIuMjM4IDQuMDMgNS4xOCA0LjA2IDguNDEtLjMzLTMuMTAzLTUuODg4LTYuMjU0LTExLjU0LTUuMTI3LTYuNCAxLjI3Ny0xMC43MzMgNy42NDUtMTUuODM0IDguNDE0LTEuMTU5LjE3NS0yLjQ2Ni4wNy0zLjM2Ny0uNTM4LTEuMDc1LS43MjYtMS4yMjctMS41NS0xLjI1NS0yLjY1Mi0uMTc5LTYuNjkgNi40MzUtMTIuMTggMTQuNjMtMTIuNzM0LTIuODgzIDEuNDA5LTUuNDM3IDMuODY1LTcuMDg5IDYuMjY4WiIgZmlsbD0idXJsKCNiKSIvPjxkZWZzPjxsaW5lYXJHcmFkaWVudCBpZD0iYSIgeDE9IjM2IiB5MT0iMjgiIHgyPSI5LjU1NiIgeTI9IjI4IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+PHN0b3Agc3RvcC1jb2xvcj0iIzAwRjhEQiIvPjxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb2xvcj0iIzA4RjhDQyIvPjwvbGluZWFyR3JhZGllbnQ+PGxpbmVhckdyYWRpZW50IGlkPSJiIiB4MT0iNCIgeTE9IjEyIiB4Mj0iMzAuNDQ0IiB5Mj0iMTIiIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIj48c3RvcCBzdG9wLWNvbG9yPSIjMDBGOERCIi8+PHN0b3Agb2Zmc2V0PSIxIiBzdG9wLWNvbG9yPSIjMDhGOENDIi8+PC9saW5lYXJHcmFkaWVudD48L2RlZnM+PC9zdmc+";
#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        let data_url = DATA_IMAGE_SVG_PNX_ICON;
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("XPNX Token"),
            symbol: String::from("XPNX"),
            icon: Some(String::from(data_url)),
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}
