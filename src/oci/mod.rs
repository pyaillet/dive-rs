pub mod image;
pub mod registry;

use std::fmt;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hash(String);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_hash_display() -> Result<(), Box<dyn error::Error>> {
        assert_eq!("fake_hash", format!("{}", Hash("fake_hash".to_string())));
        Ok(())
    }
}
