mod config;
mod layer;
mod manifest;
mod registry;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hash(String);
