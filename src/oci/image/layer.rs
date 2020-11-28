use std::error;
use std::fmt;

use flate2::read::GzDecoder;
use std::fs;
use tar::Archive;
use tar::EntryType;

#[derive(Debug)]
pub struct File {}

pub struct Dir {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Symlink {
    pub target: String,
}

pub enum NodeSpec {
    File(File),
    Dir(Dir),
    Symlink(Symlink),
    Unimplemented(()),
}

impl fmt::Debug for NodeSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeSpec::File(_) => f.debug_struct("File").finish(),
            NodeSpec::Dir(_) => f.debug_struct("Dir").finish(),
            NodeSpec::Symlink(s) => f
                .debug_struct("Symlink")
                .field("target", &s.target)
                .finish(),
            NodeSpec::Unimplemented(_) => f.debug_struct("UNIMPLEMENTED").finish(),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub full_path: String,
    pub node_type: NodeSpec,
}

pub struct Layer {
    pub file_tree: Node,
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            file_tree: Node {
                name: "/".to_string(),
                full_path: "/".to_string(),
                node_type: NodeSpec::Dir(Dir { nodes: Vec::new() }),
            },
        }
    }
}

impl Layer {
    pub fn from_tar_gz(archive_file: &str) -> Layer {
        let tar_gz = fs::File::open(archive_file).expect("Unable to open file");
        let mut archive = Archive::new(GzDecoder::new(tar_gz));
        let nodes = archive
            .entries()
            .expect("No entries found")
            .filter_map(|e| e.ok())
            .map(|entry| -> Result<Node, Box<dyn error::Error>> {
                let header = entry.header();
                let path = entry.path()?.to_owned();
                Ok(Node {
                    name: match path.file_name() {
                        Some(n) => n.to_string_lossy().to_string(),
                        None => "Not found".to_string(),
                    },
                    full_path: path.to_str().unwrap().to_string(),
                    node_type: match header.entry_type() {
                        EntryType::Regular => NodeSpec::File(File {}),
                        EntryType::Directory => NodeSpec::Dir(Dir { nodes: Vec::new() }),
                        EntryType::Symlink => NodeSpec::Symlink(Symlink {
                            target: header
                                .link_name()?
                                .unwrap()
                                .to_owned()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        }),
                        _ => NodeSpec::Unimplemented(()),
                    },
                })
            })
            .filter_map(|r| r.ok())
            .collect::<Vec<Node>>();
        Layer {
            file_tree: Node {
                name: "/".to_string(),
                full_path: "/".to_string(),
                node_type: NodeSpec::Dir(Dir { nodes }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tar_gz() -> Result<(), Box<dyn error::Error>> {
        let layer = Layer::from_tar_gz("tests/resources/tests_local_layer_tar_gzip");
        if let NodeSpec::Dir(root) = layer.file_tree.node_type {
            let node_count = root.nodes.len();
            for n in root.nodes {
                dbg!(n);
            }
            assert_eq!(node_count, 495);
        }
        Ok(())
    }
}
