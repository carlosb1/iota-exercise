use std::collections::HashMap;
use std::fmt;

use thiserror::Error;

use crate::domain::{Metrics, Transaction};

#[derive(Error, Debug, PartialEq)]
pub enum GraphError {
    #[error("duplicated id=`{0}`")]
    DuplicatedIdFound(u32),
    #[error("unknown parent")]
    ParentNotFound,
    #[error("not specified parent")]
    ParentNotSpecified,
}

//add specification
#[derive(Debug, PartialEq)]
pub struct Graph {
    pub num_nodes: u32,
    pub nodes: HashMap<u32, Transaction>,
}
const ROOT_NODE: Transaction = Transaction {
    id: 1,
    parents: None,
    timestamp: 0,
    metrics: Metrics {
        depth: 0,
        in_reference: 0,
    },
};

impl Graph {
    pub fn with_capacity(num_child: u32) -> Self {
        let num_nodes = num_child + 1;
        let mut nodes: HashMap<u32, Transaction> = HashMap::with_capacity(num_nodes as usize);
        nodes.insert(1, ROOT_NODE);
        Graph { num_nodes, nodes }
    }

    fn exists_node(&mut self, id: u32) -> bool {
        self.nodes.contains_key(&id)
    }

    fn add_vertex(&mut self, node: &Transaction) {
        self.nodes.insert(node.id, (*node).clone());
    }

    pub fn add_node(&mut self, node: &mut Transaction) -> Result<(), GraphError> {
        /*  checkers before add a node */
        if self.exists_node(node.id) {
            return Err(GraphError::DuplicatedIdFound(node.id));
        }

        if node.parents.is_none() {
            return Err(GraphError::ParentNotSpecified);
        }
        let parents = node
            .parents
            .expect("It was not checked correctly the node s parent");
        if !self.exists_node(parents.0) || !self.exists_node(parents.1) {
            return Err(GraphError::ParentNotFound);
        }

        /* setting metrics */
        self.update_metrics(node);

        /* add vertex */
        self.add_vertex(node);
        Ok(())
    }
    fn update_metrics(&mut self, node: &mut Transaction) {
        let left_parent = self
            .nodes
            .get_mut(&node.parents.unwrap().0)
            .expect("getting value for left parent");
        left_parent.metrics.in_reference += 1;
        let left_depth = left_parent.metrics.depth;

        let right_parent = self
            .nodes
            .get_mut(&node.parents.unwrap().1)
            .expect("getting value for right parent");
        right_parent.metrics.in_reference += 1;
        let right_depth = right_parent.metrics.depth;

        node.metrics.depth = std::cmp::min(left_depth, right_depth) + 1;
    }
}

impl TryFrom<Vec<(u32, u32, u32)>> for Graph {
    type Error = GraphError;
    fn try_from(values: Vec<(u32, u32, u32)>) -> Result<Graph, Self::Error> {
        let nodes = values
            .iter()
            .enumerate()
            .map(|(index, &(left_parent, right_parent, timestamp))| {
                let id = (index as u32) + 2;
                Transaction::new(id, left_parent, right_parent, timestamp)
            })
            .collect::<Vec<Transaction>>();
        let mut graph = Self::with_capacity(values.len() as u32);
        for mut node in nodes {
            graph.add_node(&mut node)?;
        }
        Ok(graph)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_nodes: Vec<(&u32, &Transaction)> = self.nodes.iter().collect();
        sorted_nodes.sort_by_key(|k| k.0);

        let mut output = String::new();
        sorted_nodes.iter().for_each(|(_, node)| {
            output += format!("{:?}\n", node).as_str();
        });
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_new_graph_with_new_element() {
        let graph = Graph::with_capacity(0);
        assert_eq!(1, graph.nodes.len());
        assert_eq!(1, *graph.nodes.keys().next().expect("Key not found"));
    }

    #[test]
    fn should_create_a_simple_graph() {
        let mut graph = Graph::with_capacity(2);
        let mut node = Transaction::new(2, 1, 1, 0);
        graph.add_node(&mut node).unwrap();
        //check graph
        let mut ids = graph.nodes.keys().collect::<Vec<&u32>>();
        ids.sort();
        assert_eq!(2, ids.len());
        assert_eq!(vec![&(1 as u32), &(2 as u32)], ids);
    }
}
