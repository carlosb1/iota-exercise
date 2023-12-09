use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GraphError {
    #[error("duplicated id=`{0}`")]
    DuplicatedIdFound(u32),
    #[error("unknown parent")]
    ParentNotFound,
    #[error("not specified parent")]
    ParentNotSpecified,
}

#[derive(Debug, Clone, PartialEq)]
struct Metrics {
    depth: u32,
    in_reference: u32,
}
impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!(
            "(depth={:},in_reference={:})",
            self.depth, self.in_reference
        );
        write!(f, "{}", output)
    }
}
// Domain classes
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: u32,
    pub timestamp: u32,
    pub parents: Option<(u32, u32)>,
    metrics: Metrics,
}

impl Node {
    fn new(id: u32, left_parent: u32, right_parent: u32, timestamp: u32) -> Self {
        Node {
            id,
            timestamp,
            parents: Some((left_parent, right_parent)),
            metrics: Metrics {
                depth: 0,
                in_reference: 0,
            },
        }
    }
}

impl TryFrom<(&[&str; 3], u32)> for Node {
    type Error = ParseIntError;
    fn try_from(params: (&[&str; 3], u32)) -> Result<Self, ParseIntError> {
        let fields = params.0;
        let id = params.1;
        let left_parent = fields[0].parse()?;
        let right_parent = fields[1].parse()?;
        let timestamp = fields[2].parse()?;
        Ok(Node::new(id as u32, left_parent, right_parent, timestamp))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        if let Some(parents) = self.parents {
            output += format!(
                "- id={:}(left={:?} right={:?}) info=(t={:?}, metrics={:})",
                self.id, parents.0, parents.1, self.timestamp, self.metrics
            )
            .as_str();
        } else {
            output += format!(
                "- id={:}() info=(t={:?}, metrics={:})",
                self.id, self.timestamp, self.metrics
            )
            .as_str();
        }
        write!(f, "{}", output)
    }
}

//add specification
#[derive(Debug, PartialEq)]
pub struct Graph {
    num_nodes: u32,
    pub nodes: HashMap<u32, Node>,
}
const ROOT_NODE: Node = Node {
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
        let mut nodes: HashMap<u32, Node> = HashMap::with_capacity(num_nodes as usize);
        nodes.insert(1, ROOT_NODE);
        Graph { num_nodes, nodes }
    }

    fn exists_node(&mut self, id: u32) -> bool {
        self.nodes.contains_key(&id)
    }

    fn add_vertex(&mut self, node: &Node) {
        self.nodes.insert(node.id, (*node).clone());
    }

    pub fn add_node(&mut self, node: &mut Node) -> Result<(), GraphError> {
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
        self.update_metrics(node, parents);

        /* add vertex */
        self.add_vertex(node);
        Ok(())
    }
    fn update_metrics(&mut self, node: &mut Node, parents: (u32, u32)) {
        let left_parent = self
            .nodes
            .get_mut(&parents.0)
            .expect("getting value for left parent");
        left_parent.metrics.in_reference += 1;
        let left_depth = left_parent.metrics.depth;

        let right_parent = self
            .nodes
            .get_mut(&parents.1)
            .expect("getting value for right parent");
        right_parent.metrics.in_reference += 1;
        let right_depth = right_parent.metrics.depth;

        node.metrics.depth = std::cmp::min(left_depth, right_depth) + 1;
    }

    pub fn average_depth(&self) -> f64 {
        self.nodes
            .values()
            .map(|node| node.metrics.depth)
            .sum::<u32>() as f64
            / self.num_nodes as f64
    }
    pub fn average_nodes_by_depth(&self) -> f64 {
        let mut score_depth = HashMap::new();
        for node in self.nodes.values().filter(|v| v.id != 1) {
            let entry = score_depth.entry(node.metrics.depth).or_insert(0);
            *entry += 1;
        }
        let num_scores = score_depth.len() as f64;
        let depths = score_depth.into_values().sum::<u32>() as f64;
        depths / num_scores
    }
    pub fn average_in_references(&self) -> f64 {
        self.nodes
            .values()
            .map(|node| node.metrics.in_reference)
            .sum::<u32>() as f64
            / self.num_nodes as f64
    }
    pub fn stats(&self) -> ResultStadistics {
        let average_depth = self.average_depth();
        let average_nodes_by_depth = self.average_nodes_by_depth();
        let average_in_references = self.average_in_references();
        ResultStadistics {
            average_depth,
            average_nodes_by_depth,
            average_in_references,
        }
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
                Node::new(id, left_parent, right_parent, timestamp)
            })
            .collect::<Vec<Node>>();
        let mut graph = Self::with_capacity(values.len() as u32);
        for mut node in nodes {
            graph.add_node(&mut node)?;
        }
        Ok(graph)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_nodes: Vec<(&u32, &Node)> = self.nodes.iter().collect();
        sorted_nodes.sort_by_key(|k| k.0);

        let mut output = String::new();
        sorted_nodes.iter().for_each(|(_, node)| {
            output += format!("{:?}\n", node).as_str();
        });
        write!(f, "{}", output)
    }
}

pub struct ResultStadistics {
    pub average_depth: f64,
    pub average_nodes_by_depth: f64,
    pub average_in_references: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    const TEST: [(u32, u32, u32); 5] = [(1, 1, 0), (1, 2, 0), (2, 2, 1), (3, 3, 2), (3, 4, 3)];

    #[test]
    fn should_create_a_new_graph_with_new_element() {
        let graph = Graph::with_capacity(0);
        assert_eq!(1, graph.nodes.len());
        assert_eq!(1, *graph.nodes.keys().next().expect("Key not found"));
    }

    //TODO builder pattern
    #[test]
    fn should_create_a_simple_graph() {
        let mut graph = Graph::with_capacity(2);
        let mut node = Node::new(2, 1, 1, 0);
        graph.add_node(&mut node).unwrap();
        //check graph
        let mut ids = graph.nodes.keys().collect::<Vec<&u32>>();
        ids.sort();
        assert_eq!(2, ids.len());
        assert_eq!(vec![&(1 as u32), &(2 as u32)], ids);
    }

    #[test]
    fn should_create_a_graph_correctly() {
        let graph = Graph::try_from(TEST.to_vec()).unwrap();
        println!("{:}", graph);
    }

    #[test]
    fn should_calculate_average_depth() {
        let graph = Graph::try_from(TEST.to_vec()).unwrap();
        let average_depth = graph.average_depth();
        assert_relative_eq!(1.33, average_depth, epsilon = 0.01);
    }
    #[test]
    fn should_calculate_number_of_nodes_by_depth() {
        let graph = Graph::try_from(TEST.to_vec()).unwrap();
        let nodes_by_depth = graph.average_nodes_by_depth();
        assert_eq!(2.5, nodes_by_depth);
    }
    #[test]
    fn should_calculate_in_percentages() {
        let graph = Graph::try_from(TEST.to_vec()).unwrap();
        let avrg_in_references = graph.average_in_references();
        assert_relative_eq!(1.66, avrg_in_references, epsilon = 0.01);
    }
}
