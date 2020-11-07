//{
//  "schemaVersion": 2,
//  "config": {
//    "mediaType": "application/vnd.oci.image.config.v1+json",
//    "size": 7023,
//    "digest": "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7"
//  },
//  "layers": [
//    {
//      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
//      "size": 32654,
//      "digest": "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0"
//    },
//    {
//      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
//      "size": 16724,
//      "digest": "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b"
//    },
//    {
//      "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
//      "size": 73109,
//      "digest": "sha256:ec4b8955958665577945c89419d1af06b5f7636b4ac3da7f12184802ad867736"
//    }
//  ],
//  "annotations": {
//    "com.example.key1": "value1",
//    "com.example.key2": "value2"
//  }
//}

use std::collections::HashMap;
use std::error;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::oci::Hash;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    media_type: String,
    size: u64,
    digest: Hash,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    schema_version: u8,
    config: Media,
    layers: Vec<Media>,
    annotations: HashMap<String, String>,
}

pub fn parse_manifest(content: &str) -> Result<Manifest, Box<dyn error::Error>> {
    let m: Manifest = serde_json::from_str(content)?;
    Ok(m)
}

#[test]
fn test_parse_manifest_ok() {
    let c = r#"
        {
            "schemaVersion": 2,
            "config": {
                "mediaType": "application/vnd.oci.image.config.v1+json",
                "size": 7023,
                "digest": "sha256:b5b2b2c507a0944348e0303114d8d93aaaa081732b86451d9bce1f432a537bc7"
            },
            "layers": [
                {
                  "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
                  "size": 32654,
                  "digest": "sha256:9834876dcfb05cb167a5c24953eba58c4ac89b1adf57f28f2f9d09af107ee8f0"
                },
                {
                  "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
                  "size": 16724,
                  "digest": "sha256:3c3a4604a545cdc127456d94e421cd355bca5b528f4a9c1905b15da2eb4a4c6b"
                }
            ],
            "annotations": {
              "annot1": "value1",
              "annot2": "value2"
            }
        }"#;
    let m = parse_manifest(c);

    if m.is_err() {
        println!("Error {:?}", m.err());
        assert!(false);
    } else {
        assert!(m.is_ok());
        println!("Result: {:?}", m.unwrap());
    }
}
