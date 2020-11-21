pub mod config;
pub mod layer;
pub mod manifest;

use std::str::FromStr;
use url::{ParseError, Url};

pub trait Reference {
    fn hostport(&self) -> String;

    fn fullname(&self) -> String;

    fn name(&self) -> String;

    fn tag(&self) -> String;

    fn digest(&self) -> String;

    fn scheme(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageReference(pub Url);

impl FromStr for ImageReference {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sanatized = if s.starts_with("http://") {
            s.to_string()
        } else if s.starts_with("https://") {
            s.to_string()
        } else {
            let host_prepended = match s.find("/") {
                Some(x) => match s[0..x].find(".") {
                    Some(_) => s.to_string(),
                    None => format!("docker.io/{}", s),
                },
                None => format!("docker.io/{}", s),
            };
            format!("https://{}", host_prepended)
        };
        let parsed = Url::parse(&sanatized)?;
        Ok(ImageReference(parsed))
    }
}

impl Reference for ImageReference {
    fn hostport(&self) -> String {
        let host = match self.0.host() {
            Some(x) => x.to_string(),
            None => "docker.io".to_string(),
        };
        let port = match self.0.port() {
            Some(x) => x,
            None => 443,
        };
        format!("{}:{}", host, port).to_string()
    }

    fn fullname(&self) -> String {
        let fullname = self.0.path()[1..].to_string();
        dbg!(fullname);
        self.0.path()[1..].to_string()
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

    fn scheme(&self) -> String {
        self.0.scheme().to_string()
    }

    fn digest(&self) -> String {
        let path = self.0.path();
        match path.find("@") {
            Some(x) => path[x..].to_string(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hostport_ok() {
        assert_eq!(
            "registry.my:5000/test/test:v1"
                .parse::<ImageReference>()
                .unwrap()
                .hostport(),
            "registry.my:5000".to_string()
        );
        assert_eq!(
            "test/test".parse::<ImageReference>().unwrap().hostport(),
            "docker.io:443".to_string()
        );
    }

    #[test]
    fn tag_ok() {
        assert_eq!(
            "registry.my:5000/test/test:v1"
                .parse::<ImageReference>()
                .unwrap()
                .tag(),
            "v1".to_string()
        );
        assert_eq!(
            "registry.my:5000/test/test"
                .parse::<ImageReference>()
                .unwrap()
                .tag(),
            "latest".to_string()
        );
    }

    #[test]
    fn name_ok() {
        assert_eq!(
            "registry.my:5000/test/test:v1"
                .parse::<ImageReference>()
                .unwrap()
                .name(),
            "test/test".to_string()
        );
        assert_eq!(
            "registry.my:5000/test/test"
                .parse::<ImageReference>()
                .unwrap()
                .name(),
            "test/test".to_string()
        );
    }

    #[test]
    fn fullname_ok() {
        assert_eq!(
            "registry.my:5000/test/test:v1"
                .parse::<ImageReference>()
                .unwrap()
                .fullname(),
            "test/test:v1".to_string()
        );
        assert_eq!(
            "registry.my:5000/test/test"
                .parse::<ImageReference>()
                .unwrap()
                .fullname(),
            "test/test".to_string()
        );
    }

    #[test]
    fn scheme_ok() {
        assert_eq!(
            "registry.my:5000/test/test:v1"
                .parse::<ImageReference>()
                .unwrap()
                .scheme(),
            "https".to_string()
        );
        assert_eq!(
            "http://registry.my:5000/test/test:v1"
                .parse::<ImageReference>()
                .unwrap()
                .scheme(),
            "http".to_string()
        );
        assert_eq!(
            "test/test:v1".parse::<ImageReference>().unwrap().scheme(),
            "https".to_string()
        );
    }
}
