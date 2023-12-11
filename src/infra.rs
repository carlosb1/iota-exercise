// Infrastructure module. Following the DDD - clean architecture, It
// includes connected modules to external services. In this case, it only
// needs a DB that it is designed as a repository pattern.
//
// This DB repository checks the filepath consistency and load the graph,
// for this use case, it only needs this function but this design is open
// for extension.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::graph::Graph;

use thiserror::Error;

/// Set of possible infrastructure errors.
#[derive(Error, Debug, PartialEq)]
pub enum InfraError {
    #[error("not correct node format")]
    ParseTransaction,
    #[error("not correct graph parse :`{0}`")]
    ParseGraph(String),
    #[error("not correct path file")]
    NotFileSpecified,
}

fn parse_node(line: String) -> Result<(u32, u32, u32), InfraError> {
    let fields: [&str; 3] = line
        .split(' ')
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|_| InfraError::ParseTransaction)?;
    let left_parent = fields[0]
        .parse()
        .map_err(|_| InfraError::ParseTransaction)?;
    let right_parent = fields[1]
        .parse()
        .map_err(|_| InfraError::ParseTransaction)?;
    let timestamp = fields[2]
        .parse()
        .map_err(|_| InfraError::ParseTransaction)?;
    Ok((left_parent, right_parent, timestamp))
}

/// Public repository structure, it includes the `path_buf`
/// for the database.
pub struct DBRepository {
    path_buf: PathBuf,
}

impl DBRepository {
    /// Constructor function for `path_str`. It validates is correct
    pub fn new(path_str: &str) -> Option<Self> {
        let path_buf = PathBuf::from(path_str);
        if !path_buf.exists() {
            return None;
        }
        let repo = DBRepository { path_buf };
        Some(repo)
    }

    /// Graph load function. It throws different errors if something works
    /// wrong (File is removed or modified).
    pub fn load(&self) -> Result<Graph, InfraError> {
        let file = File::open(self.path_buf.clone()).map_err(|_| InfraError::NotFileSpecified)?;
        let reader = BufReader::new(file);

        let mut nodes: Vec<(u32, u32, u32)> = Vec::new();
        for (num, line) in reader.lines().enumerate() {
            match num {
                0 => {
                    line.expect("First line was not parsed")
                        .parse::<u32>()
                        .map_err(|_| {
                            InfraError::ParseGraph("first line was not parsed".to_string())
                        })?;
                }
                _ => {
                    let line = line.expect("Failed to read line");
                    nodes.push(parse_node(line)?);
                }
            }
        }
        let graph = Graph::try_from(nodes)
            .map_err(|_| InfraError::ParseGraph("impossible add node in the graph".to_string()))?;

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;
    use tempfile::TempDir;

    use crate::domain::Transaction;

    fn create_temp_file(input_content: &str, dir: &TempDir) -> PathBuf {
        let file_path = dir.path().join("temp.txt");
        let mut file = File::create(file_path.clone()).unwrap();
        file.write_all(input_content.as_bytes()).unwrap();
        file_path
    }

    #[test]
    fn should_load_all_database_file() {
        let input_content: &str = "5\n1 1 0\n1 2 0\n2 2 1\n3 3 2\n3 4 3";
        let dir = tempdir().unwrap();
        let file_path = create_temp_file(input_content, &dir);
        let repo = DBRepository::new(file_path.to_str().unwrap()).unwrap();

        let graph = repo.load().unwrap();

        let mut sorted_nodes = graph.nodes.iter().collect::<Vec<(&u32, &Transaction)>>();
        sorted_nodes.sort_by_key(|(&key, _)| key);
        assert_eq!(6, sorted_nodes.len());
    }

    #[test]
    fn should_load_a_node_from_database_file() {
        let input_content: &str = "1\n1 1 0";
        let dir = tempdir().unwrap();
        let file_path = create_temp_file(input_content, &dir);
        let repo = DBRepository::new(file_path.to_str().unwrap()).unwrap();

        let graph = repo.load().unwrap();

        let mut sorted_nodes = graph.nodes.iter().collect::<Vec<(&u32, &Transaction)>>();
        sorted_nodes.sort_by_key(|(&key, _)| key);
        assert_eq!(2, sorted_nodes.len());
        assert_eq!(2, *sorted_nodes.get(1).unwrap().0);
        assert_eq!(Some((1, 1)), (*sorted_nodes.get(1).unwrap().1).parents);
        assert_eq!(2, (*sorted_nodes.get(1).unwrap().1).id);
        assert_eq!(0, (*sorted_nodes.get(1).unwrap().1).timestamp);
    }

    #[test]
    fn should_fail_parse_num_lines() {
        let input_content: &str = "xx\n1 1 0";
        let dir = tempdir().unwrap();
        let file_path = create_temp_file(input_content, &dir);
        let repo = DBRepository::new(file_path.to_str().unwrap()).unwrap();
        assert_eq!(
            Err(InfraError::ParseGraph(
                "first line was not parsed".to_string()
            )),
            repo.load()
        );
    }

    #[test]
    fn should_fail_parse_nodes() {
        let input_content: &str = "1\n1 x";
        let dir = tempdir().unwrap();
        let file_path = create_temp_file(input_content, &dir);
        let repo = DBRepository::new(file_path.to_str().unwrap()).unwrap();
        assert_eq!(Err(InfraError::ParseTransaction), repo.load());
    }

    #[test]
    fn should_fail_open_file() {
        let repo = DBRepository::new("notexist");
        assert!(repo.is_none());
    }
}
