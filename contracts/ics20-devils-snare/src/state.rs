// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cw_storage_plus::Item;

pub const CHANNEL_ID: Item<String> = Item::new("channel_id");
pub const BRIDGE_CONTRACT: Item<String> = Item::new("bridge_contract");
pub const RECEIVER: Item<String> = Item::new("receiver");
