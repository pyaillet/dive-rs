use std::fmt;

pub struct Layer {}

#[derive(Debug)]
struct File {}

struct Dir {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
struct Symlink {
    pub target: String,
}

enum NodeSpec {
    File(File),
    Dir(Dir),
    Symlink(Symlink),
    Unimplemented(()),
}

impl fmt::Debug for NodeSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
struct Node {
    pub name: String,
    pub full_path: String,
    pub node_type: NodeSpec,
}

#[cfg(test)]
mod tests {
    use super::*;

    use flate2::read::GzDecoder;
    use std::error;
    use std::fs;
    use tar::Archive;
    use tar::EntryType;

    #[test]
    fn test_targz() -> Result<(), Box<dyn error::Error>> {
        let tar_gz = fs::File::open("tests/resources/tests_local_layer_tar_gzip")?;
        let mut archive = Archive::new(GzDecoder::new(tar_gz));
        println!("Extracted the following files:");
        archive
            .entries()?
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
            .for_each(|r| {
                if let Ok(e) = r {
                    dbg!(e);
                }
            });
        Ok(())
    }
}
