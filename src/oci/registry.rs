use crate::oci::image::config::Config;
use crate::oci::image::manifest;
use crate::oci::image::manifest::Manifest;
use crate::oci::image::ImageReference;
use crate::oci::image::Reference;
use std::error;

fn manifest_url(im: ImageReference) -> String {
    format!(
        "{}://{}/v2/{}/manifests/{}",
        im.scheme(),
        im.hostport(),
        im.name(),
        im.tag()
    )
}

fn blob_url(im: ImageReference, digest: String) -> String {
    format!(
        "{}://{}/v2/{}/blobs/{}",
        im.scheme(),
        im.hostport(),
        im.name(),
        digest
    )
}

pub fn get_manifest(im: ImageReference) -> Result<Manifest, Box<dyn error::Error>> {
    let url = &manifest_url(im);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::ACCEPT, manifest::MANIFEST_MIME)
        .send()?;
    let m: Manifest = serde_json::from_str(&res.text()?)?;

    Ok(m)
}

pub fn get_config(im: ImageReference) -> Result<Config, Box<dyn error::Error>> {
    let m = get_manifest(im.clone())?;
    let url = &blob_url(im, (&m.config.digest).to_string());
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

    use httpmock::Method::GET;
    use httpmock::MockServer;

    use crate::oci::image::config;
    use std::str::FromStr;

    #[test]
    fn get_manifest_ok() -> Result<(), Box<dyn error::Error>> {
        let server = MockServer::start();
        let mock_manifest = server.mock(|when, then| {
            when.method(GET).path("/v2/test/manifests/local");
            then.status(200)
                .body_from_file("tests/resources/tests_local_manifest.json");
        });
        let image_url = format!("http://{}:{}/test:local", server.host(), server.port());

        let image_ref = ImageReference::from_str(&image_url)?;

        let r: Manifest = get_manifest(image_ref)?;

        mock_manifest.assert();
        assert_eq!(r.schema_version, 2);

        Ok(())
    }

    #[test]
    fn get_config_ok() -> Result<(), Box<dyn error::Error>> {
        let server = MockServer::start();
        let mock_manifest = server.mock(|when, then| {
            when.method(GET).path("/v2/test/manifests/local");
            then.status(200)
                .body_from_file("tests/resources/tests_local_manifest.json");
        });
        let mock_config = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/test/blobs/sha256:d6e46aa2470df1d32034c6707c8041158b652f38d2a9ae3d7ad7e7532d22ebe0");
            then.status(200)
                .body_from_file("tests/resources/tests_local_config.json");
        });

        let image_url = format!("http://{}:{}/test:local", server.host(), server.port());

        let image_ref = ImageReference::from_str(&image_url)?;

        let r: Config = get_config(image_ref)?;

        mock_manifest.assert();
        mock_config.assert();
        assert_eq!(r.os, config::OS::Linux);
        assert_eq!(r.architecture, config::Architecture::Amd64);

        Ok(())
    }

    #[test]
    fn get_layer_ok() -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
