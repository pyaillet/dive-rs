pub mod config;
pub mod layer;
pub mod manifest;

use std::error;

pub trait Reference {
    fn registry(self) -> Result<String, Box<dyn error::Error>>;

    fn fullname(self) -> Result<String, Box<dyn error::Error>>;

    fn tag(self) -> Result<String, Box<dyn error::Error>>;

    fn digest(self) -> Result<String, Box<dyn error::Error>>;
}

#[derive(Debug, PartialEq)]
pub struct ImageReference(String);

impl Reference for ImageReference {
    fn registry(self) -> Result<String, Box<dyn error::Error>> {
        match self.0.find("/") {
            Some(x) => Ok(self.0[..x].to_string()),
            None => Ok("docker.io".to_string()),
        }
    }

    fn fullname(self) -> Result<String, Box<dyn error::Error>> {
        match self.0.find("/") {
            Some(x) => Ok(self.0[x + 1..].to_string()),
            None => Ok(self.0),
        }
    }

    fn tag(self) -> Result<String, Box<dyn error::Error>> {
        match self.0.rfind("/") {
            Some(x) => match self.0[x..].find(":") {
                Some(y) => Ok(self.0[x + y + 1..].to_string()),
                None => Ok("latest".to_string()),
            },
            None => Ok("".to_string()),
        }
    }

    fn digest(self) -> Result<String, Box<dyn error::Error>> {
        match self.0.find("@") {
            Some(x) => Ok(self.0[x..].to_string()),
            None => Ok("".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_ok() {
        assert_eq!(
            ImageReference("localhost:5000/test/test:v1".to_string())
                .tag()
                .unwrap(),
            "v1".to_string()
        );
        assert_eq!(
            ImageReference("localhost:5000/test/test".to_string())
                .tag()
                .unwrap(),
            "latest".to_string()
        );
    }

    #[test]
    fn fullname_ok() {
        assert_eq!(
            ImageReference("localhost:5000/test/test:v1".to_string())
                .fullname()
                .unwrap(),
            "test/test:v1".to_string()
        );
        assert_eq!(
            ImageReference("localhost:5000/test/test".to_string())
                .fullname()
                .unwrap(),
            "test/test".to_string()
        );
    }

    #[test]
    fn registry_ok() {
        let image = ImageReference("localhost:5000/myorg/myimage/test:local".to_string());

        let r = image.registry();

        assert!(r.is_ok(), "Manifest parsing failed: `{}`", r.err().unwrap());
        assert_eq!(r.unwrap(), "localhost:5000");
    }
}
