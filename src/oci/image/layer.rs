use std::error;
use std::fmt;

use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use tar::Archive;
use tar::EntryType;

#[derive(Debug)]
pub struct File {}

#[derive(Debug)]
pub struct Dir {}

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
    pub children: Option<Vec<Node>>,
    pub node_type: NodeSpec,
}

impl Node {
    fn new_file(full_path: String) -> Node {
        let p = Path::new(&full_path);
        Node {
            name: p
                .file_name()
                .unwrap_or(OsStr::new("Not found"))
                .to_string_lossy()
                .to_owned()
                .to_string(),
            full_path,
            children: None,
            node_type: NodeSpec::File(File {}),
        }
    }

    fn add_node(&mut self, new_node: Node) {
        match self.children {
            Some(ref mut x) => x.push(new_node),
            None => (),
        }
    }
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
                children: Some(Vec::new()),
                node_type: NodeSpec::Dir(Dir {}),
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
                    full_path: format!("/{}", path.to_owned().to_string_lossy()),
                    node_type: match header.entry_type() {
                        EntryType::Regular => NodeSpec::File(File {}),
                        EntryType::Directory => NodeSpec::Dir(Dir {}),
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
                    children: Some(Vec::new()),
                })
            })
            .filter_map(|r| r.ok())
            .collect::<Vec<Node>>();
        let l = Layer {
            file_tree: Node {
                name: "/".to_string(),
                full_path: "/".to_string(),
                node_type: NodeSpec::Dir(Dir {}),
                children: Some(nodes),
            },
        };
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tar_gz() -> Result<(), Box<dyn error::Error>> {
        let layer = Layer::from_tar_gz("tests/resources/tests_local_layer_tar_gzip");
        let nodes = &layer.file_tree.children.unwrap();
        for n in nodes {
            dbg!(n);
        }
        assert_eq!(nodes.len(), 495);
        Ok(())
    }

    #[test]
    fn test_add_node() -> Result<(), Box<dyn error::Error>> {
        let mut layer: Layer = Default::default();
        let n = Node::new_file("app".to_string());
        layer.file_tree.add_node(n);
        let c = &layer.file_tree.children.unwrap();
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].name, "app".to_string());
        Ok(())
    }
}
