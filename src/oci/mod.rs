pub mod image;
pub mod registry;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hash(String);

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
