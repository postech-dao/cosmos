use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;
use pdao_colony_contract_common::light_client::LightClient;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub light_client: LightClient,
}

pub const STATE: Item<State> = Item::new("state");
