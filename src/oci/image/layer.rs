use std::fmt;

pub struct Layer {}

#[derive(Debug)]
struct File {
    pub name: String,
    pub full_path: String,
}

struct Dir {
    pub name: String,
    pub full_path: String,
    pub nodes: Vec<Node>,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.full_path)
    }
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name => {}\nfull_path => {}\n",
            self.name, self.full_path
        )
    }
}

#[derive(Debug)]
struct Symlink {
    pub name: String,
    pub full_path: String,
    pub target: String,
}

enum Node {
    File(File),
    Dir(Dir),
    Symlink(Symlink),
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
                let name = match path.file_name() {
                    Some(n) => n.to_string_lossy().to_string(),
                    None => "Not found".to_string(),
                };
                let full_path = path.to_str().unwrap().to_string();
                match header.entry_type() {
                    EntryType::Regular => Ok(Node::File(File {
                        name: name,
                        full_path: full_path,
                    })),
                    EntryType::Directory => Ok(Node::Dir(Dir {
                        name: name,
                        full_path: full_path,
                        nodes: Vec::new(),
                    })),
                    EntryType::Symlink => Ok(Node::Symlink(Symlink {
                        name: name,
                        full_path: full_path,
                        target: header
                            .link_name()?
                            .unwrap()
                            .to_owned()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    })),
                    _ => Err("Not found".into()),
                }
            })
            .for_each(|r| match r {
                Ok(n) => match n {
                    Node::Symlink(s) => {
                        dbg!(s);
                    }
                    Node::Dir(d) => {
                        dbg!(d);
                    }
                    Node::File(f) => {
                        dbg!(f);
                    }
                },
                Err(e) => print!("Error: {:?}", e),
            });
        Ok(())
    }
}
