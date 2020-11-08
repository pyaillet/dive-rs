use crate::oci::image::manifest::Manifest;
use std::error;

pub fn get_manifest(path: String) -> Result<Manifest, Box<dyn error::Error>> {
    let res = reqwest::blocking::get("http://httpbin.org/get")?;
    let m: Manifest = serde_json::from_str(&res.text()?)?;
    Ok(m)
}

#[test]
fn get_ok() {
    let r = get_manifest("localhost:5000/test:local".to_string());

    if r.is_err() {
        println!("Error: {:?}", r.err());
    } else {
        assert!(r.is_ok());
        println!("Resulst: {:?}", r.ok());
    }
}
