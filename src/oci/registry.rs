use crate::oci::image::manifest::ManifestV1;
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

pub fn get_manifest(path: &str) -> Result<ManifestV1, Box<dyn error::Error>> {
    let url = &manifest_url(path.to_string());
    println!("{:?}", url);
    let res = reqwest::blocking::get(url)?;
    let m: ManifestV1 = serde_json::from_str(&res.text()?)?;
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::oci::image::manifest;

    #[test]
    fn get_manifest_ok() -> Result<(), Box<dyn error::Error>> {
        let r: ManifestV1 = get_manifest("localhost:5000/test:local")?;
        assert_eq!(r.tag, "local");
        assert_eq!(r.name, "test");
        assert_eq!(r.schema_version, 1);
        assert_eq!(r.architecture, manifest::Architecture::Amd64);

        Ok(())
    }
}
