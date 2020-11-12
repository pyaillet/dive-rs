use crate::oci::image::config;
use crate::oci::image::config::Config;
use crate::oci::image::manifest;
use crate::oci::image::manifest::Manifest;
use crate::oci::image::ImageReference;
use crate::oci::image::Reference;
use std::error;

fn manifest_url(path: String) -> String {
    let im = ImageReference(path);
    format!(
        "https://{}/v2/{}/manifests/{}",
        im.registry(),
        im.name(),
        im.tag()
    )
}

fn config_url(path: String, digest: String) -> String {
    let im = ImageReference(path);
    format!(
        "https://{}/v2/{}/blobs/{}",
        im.registry(),
        im.name(),
        digest
    )
}

pub fn get_manifest(path: &str) -> Result<Manifest, Box<dyn error::Error>> {
    let url = &manifest_url(path.to_string());
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::ACCEPT, manifest::MANIFEST_MIME)
        .send()?;
    let m: Manifest = serde_json::from_str(&res.text()?)?;

    Ok(m)
}

pub fn get_config(path: &str) -> Result<Config, Box<dyn error::Error>> {
    let m = get_manifest(&path)?;
    let url = &config_url(path.to_string(), (&m.config.digest).to_string());
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::ACCEPT, manifest::CONFIG_MIME)
        .send()?;
    let c: Config = serde_json::from_str(&res.text()?)?;

    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_manifest_ok() -> Result<(), Box<dyn error::Error>> {
        let r: Manifest = get_manifest("localhost:5000/test:local")?;
        assert_eq!(r.schema_version, 2);

        Ok(())
    }

    #[test]
    fn get_config_ok() -> Result<(), Box<dyn error::Error>> {
        let r: Config = get_config("localhost:5000/test:local")?;

        assert_eq!(r.os, config::OS::Linux);
        assert_eq!(r.architecture, config::Architecture::Amd64);

        Ok(())
    }
}
