use crate::oci::image::manifest;
use crate::oci::image::manifest::Manifest;
use std::error;

pub fn get(path: String) -> Result<Manifest, Box<dyn error::Error>> {
    let res = ureq::get("http://httpbin.org/get").call().into_string()?;

    manifest::from_str(&res)
}

#[test]
fn get_ok() {
    let r = get("localhost:5000/test:local".to_string());

    if r.is_err() {
        println!("Error: {:?}", r.err());
    } else {
        assert!(r.is_ok());
        println!("Resulst: {:?}", r.ok());
    }
}
