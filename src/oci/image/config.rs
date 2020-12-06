use serde;
use serde_json::value::Value;
use std::{collections::HashMap, fmt};

use crate::oci::Hash;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OS {
    Linux,
    Windows,
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                OS::Linux => "Linux",
                OS::Windows => "Windows",
            }
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Architecture {
    Amd64,
    Aarch64,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Architecture::Aarch64 => "Aarch64",
                Architecture::Amd64 => "Amd64",
            }
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RootFSType {
    Layers,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Configuration {
    pub user: Option<String>,
    pub exposed_ports: Option<HashMap<String, Value>>,
    pub env: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub volumes: Option<HashMap<String, Value>>,
    pub working_dir: Option<String>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RootFS {
    pub diff_ids: Vec<Hash>,
    #[serde(alias = "type")]
    pub rootfs_type: RootFSType,
}

fn default_as_false() -> bool {
    false
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct History {
    pub created: String,
    pub created_by: String,
    #[serde(default = "default_as_false")]
    pub empty_layer: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub created: String,
    pub author: Option<String>,
    pub architecture: Architecture,
    pub os: OS,
    pub config: Configuration,
    pub rootfs: RootFS,
    pub history: Vec<History>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn test_from_str_ok() -> Result<(), Box<dyn error::Error>> {
        let c = r#"
        {
            "created": "2015-10-31T22:22:56.015925234Z",
            "author": "Alyssa P. Hacker <alyspdev@example.com>",
            "architecture": "amd64",
            "os": "linux",
            "config": {
                "User": "alice",
                "ExposedPorts": {
                    "8080/tcp": {}
                },
                "Env": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "FOO=oci_is_a",
                    "BAR=well_written_spec"
                ],
                "Entrypoint": [
                    "/bin/my-app-binary"
                ],
                "Cmd": [
                    "--foreground",
                    "--config",
                    "/etc/my-app.d/default.cfg"
                ],
                "Volumes": {
                    "/var/job-result-data": {},
                    "/var/log/my-app-logs": {}
                },
                "WorkingDir": "/home/alice",
                "Labels": {
                    "com.example.project.git.url": "https://example.com/project.git",
                    "com.example.project.git.commit": "45a939b2999782a3f005621a8d0f29aa387e1d6b"
                }
            },
            "rootfs": {
              "diff_ids": [
                "sha256:c6f988f4874bb0add23a778f753c65efe992244e148a1d2ec2a8b664fb66bbd1",
                "sha256:5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef"
              ],
              "type": "layers"
            },
            "history": [
              {
                "created": "2015-10-31T22:22:54.690851953Z",
                "created_by": "/bin/sh -c #(nop) ADD file:a3bc1e842b69636f9df5256c49c5374fb4eef1e281fe3f282c65fb853ee171c5 in /"
              },
              {
                "created": "2015-10-31T22:22:55.613815829Z",
                "created_by": "/bin/sh -c #(nop) CMD [\"sh\"]",
                "empty_layer": true
              }
            ]
        }"#;

        let c: Config = serde_json::from_str(c)?;
        assert_eq!(c.config.user, Some("alice".to_string()));
        assert_eq!(c.config.working_dir, Some("/home/alice".to_string()));
        assert_eq!(c.config.labels.unwrap().len(), 2);
        assert_eq!(c.history.len(), 2);
        assert_eq!(c.rootfs.diff_ids.len(), 2);
        assert_eq!(
            c.config.entrypoint,
            Some(vec!["/bin/my-app-binary".to_string()])
        );
        Ok(())
    }

    #[test]
    fn test_display_architecture() -> Result<(), Box<dyn error::Error>> {
        assert_eq!("Amd64", format!("{}", Architecture::Amd64));
        assert_eq!("Aarch64", format!("{}", Architecture::Aarch64));
        Ok(())
    }

    #[test]
    fn test_display_os() -> Result<(), Box<dyn error::Error>> {
        assert_eq!("Linux", format!("{}", OS::Linux));
        assert_eq!("Windows", format!("{}", OS::Windows));
        Ok(())
    }
}
