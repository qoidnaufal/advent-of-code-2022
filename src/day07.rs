use std::collections::HashMap;

#[allow(dead_code)]
static INPUT: &'static str = include_str!("./day07.txt");

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct NodeId(usize);

#[derive(Debug)]
struct Tree<T> {
    nodes: Vec<T>,
    children: HashMap<NodeId, Vec<NodeId>>,
    parents: HashMap<NodeId, NodeId>,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Self {
            nodes: Default::default(),
            children: Default::default(),
            parents: Default::default(),
        }
    }

    fn get(&self, id: NodeId) -> Option<&T> {
        self.nodes.get(id.0)
    }

    fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        self.nodes.get_mut(id.0)
    }

    fn iter_nodes(&self) -> impl Iterator<Item = NodeId> {
        (0..self.nodes.len()).map(|i| NodeId(i))
    }

    fn iter_children(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.children
            .get(&id)
            .into_iter()
            .flat_map(|vec| vec.iter().copied())
    }

    fn get_parent(&self, id: NodeId) -> Option<NodeId> {
        self.parents.get(&id).copied()
    }

    fn push(&mut self, parent: Option<NodeId>, node: T) -> Option<NodeId> {
        if let Some(parent) = parent {
            if self.get(parent).is_none() {
                return None;
            }
        }

        let id = NodeId(self.nodes.len());
        self.nodes.push(node);

        if let Some(parent) = parent {
            self.children.entry(parent).or_default().push(id);
            self.parents.insert(id, parent);
        }

        Some(id)
    }
}

impl Tree<Directory<'_>> {
    fn get_total_size(&self, id: NodeId) -> Option<u64> {
        self.get(id)
            .map(|dir| dir.files.iter().map(|file| file.size).sum::<u64>())
            .map(|file_size| {
                self.iter_children(id)
                    .filter_map(|sub_id| self.get_total_size(sub_id))
                    .sum::<u64>()
                    + file_size
            })
    }
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: u64,
}

impl<'a> File<'a> {
    fn new(name: &'a str, size: u64) -> Self {
        Self { name, size }
    }
}

#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            files: vec![],
        }
    }
}

#[derive(Debug)]
enum ParserError {
    UnknownCommand,
    DirectoryNotFound,
    InvalidEntry,
}

fn parse_input(input: &str) -> Result<(Tree<Directory<'_>>, NodeId), ParserError> {
    let mut result = Tree::new();
    let root_id = result.push(None, Directory::new("")).unwrap();
    let mut current_directory = root_id;

    for line in input.lines() {
        if line.is_empty() {
            // ignore
            todo!()
        } else if line.starts_with("$ ") {
            // parse command
            let mut tokens = line[2..].split_ascii_whitespace();
            let cmd = tokens.next();
            let arg1 = tokens.next();

            if let (Some(cmd), Some(arg1)) = (cmd, arg1) {
                if cmd == "cd" {
                    if arg1 == "/" {
                        current_directory = root_id;
                    } else if arg1 == ".." {
                        current_directory = result
                            .get_parent(current_directory)
                            .ok_or(ParserError::DirectoryNotFound)?;
                    } else {
                        current_directory = result
                            .iter_children(current_directory)
                            .filter_map(|id| result.get(id).map(|dir| (id, dir)))
                            .find(|(_, dir)| dir.name == arg1)
                            .map(|(id, _)| id)
                            .ok_or(ParserError::DirectoryNotFound)?;
                    }
                } else {
                    return Err(ParserError::UnknownCommand);
                }
            }
        } else if line.starts_with("dir ") {
            // add directory
            let name = &line[4..];
            let directory = Directory::new(name);

            result.push(Some(current_directory), directory);
        } else {
            // add file
            let mut tokens = line.split_ascii_whitespace();
            let size = tokens.next();
            let name = tokens.next();

            if let (Some(size), Some(name)) = (size.and_then(|x| x.parse::<u64>().ok()), name) {
                let file = File::new(name, size);

                if let Some(directory) = result.get_mut(current_directory) {
                    directory.files.push(file)
                } else {
                    return Err(ParserError::DirectoryNotFound);
                }
            } else {
                return Err(ParserError::InvalidEntry);
            }
        }
    }

    Ok((result, root_id))
}

#[test]
fn part1() -> Result<(), ParserError> {
    let (fs, _) = parse_input(INPUT)?;

    let result = fs
        .iter_nodes()
        .filter_map(|id| fs.get_total_size(id))
        .filter(|size| *size <= 100000)
        .sum::<u64>();

    println!("{:?}", result);

    Ok(())
}

#[test]
fn part2() -> Result<(), ParserError> {
    let (fs, root_id) = parse_input(INPUT)?;
    let total_size = 70_000_000;
    let required_space = 30_000_000;

    let used_size = fs.get_total_size(root_id).unwrap();
    let free_size = total_size - used_size;
    let size_to_delete = required_space - free_size;

    let result = fs
        .iter_nodes()
        .filter_map(|id| fs.get_total_size(id))
        .filter(|size| *size >= size_to_delete)
        .min()
        .unwrap();

    println!("{}", result);

    Ok(())
}
