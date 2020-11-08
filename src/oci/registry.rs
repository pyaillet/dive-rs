use crate::oci::image::manifest::Manifest;
use crate::oci::image::ImageReference;
use crate::oci::image::Reference;
use std::error;

fn manifest_url(path: String) -> String {
    let im = ImageReference(path);
    let r = im.registry();
    let f = im.fullname();
    let t = im.tag();
    format!("https://{}/v2/{}/manifest/{}", r, f, t)
}

pub fn get_manifest(path: &str) -> Result<Manifest, Box<dyn error::Error>> {
    let url = &manifest_url(path.to_string());
    println!("{:?}", url);
    let res = reqwest::blocking::get(url)?;
    let m: Manifest = serde_json::from_str(&res.text()?)?;
    Ok(m)
}

#[test]
fn get_manifest_ok() {
    let r = get_manifest("localhost:5000/test:local");

    if r.is_err() {
        println!("Error: {:?}", r.err());
    } else {
        assert!(r.is_ok());
        println!("Resulst: {:?}", r.ok());
    }
}
