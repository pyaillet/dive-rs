pub mod config;
pub mod layer;
pub mod manifest;

pub trait Reference {
    fn registry(&self) -> String;

    fn fullname(&self) -> String;

    fn name(&self) -> String;

    fn tag(&self) -> String;

    fn digest(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub struct ImageReference(pub String);

impl Reference for ImageReference {
    fn registry(&self) -> String {
        match self.0.find("/") {
            Some(x) => self.0[..x].to_string(),
            None => "docker.io".to_string(),
        }
    }

    fn fullname(&self) -> String {
        match self.0.find("/") {
            Some(x) => self.0[x + 1..].to_string(),
            None => self.0.to_string(),
        }
    }

    fn name(&self) -> String {
        let fullname = self.fullname();
        match fullname.find(":") {
            Some(x) => fullname[..x].to_string(),
            None => fullname.to_string(),
        }
    }

    fn tag(&self) -> String {
        let fullname = self.fullname();
        match fullname.find(":") {
            Some(x) => fullname[x + 1..].to_string(),
            None => "latest".to_string(),
        }
    }

    fn digest(&self) -> String {
        match self.0.find("@") {
            Some(x) => self.0[x..].to_string(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_ok() {
        assert_eq!(
            ImageReference("localhost:5000/test/test:v1".to_string()).tag(),
            "v1".to_string()
        );
        assert_eq!(
            ImageReference("localhost:5000/test/test".to_string()).tag(),
            "latest".to_string()
        );
    }

    #[test]
    fn name_ok() {
        assert_eq!(
            ImageReference("localhost:5000/test/test:v1".to_string()).name(),
            "test/test".to_string()
        );
        assert_eq!(
            ImageReference("localhost:5000/test/test".to_string()).name(),
            "test/test".to_string()
        );
    }

    #[test]
    fn fullname_ok() {
        assert_eq!(
            ImageReference("localhost:5000/test/test:v1".to_string()).fullname(),
            "test/test:v1".to_string()
        );
        assert_eq!(
            ImageReference("localhost:5000/test/test".to_string()).fullname(),
            "test/test".to_string()
        );
    }

    #[test]
    fn registry_ok() {
        assert_eq!(
            ImageReference("localhost:5000/myorg/myimage/test:local".to_string()).registry(),
            "localhost:5000"
        );
    }
}
