pub mod dto {
    pub struct Stadistics {
        pub average_depth: f64,
        pub average_nodes_by_depth: f64,
        pub average_in_references: f64,
    }
}

pub mod stadistics {
    use super::dto;
    use crate::graph::Graph;
    use std::collections::HashMap;

    fn average_depth(graph: &Graph) -> f64 {
        graph
            .nodes
            .values()
            .map(|node| node.metrics.depth)
            .sum::<u32>() as f64
            / graph.num_nodes as f64
    }
    fn average_nodes_by_depth(graph: &Graph) -> f64 {
        let mut score_depth = HashMap::new();
        for node in graph.nodes.values().filter(|v| v.id != 1) {
            let entry = score_depth.entry(node.metrics.depth).or_insert(0);
            *entry += 1;
        }
        let num_scores = score_depth.len() as f64;
        let depths = score_depth.into_values().sum::<u32>() as f64;
        depths / num_scores
    }
    fn average_in_references(graph: &Graph) -> f64 {
        graph
            .nodes
            .values()
            .map(|node| node.metrics.in_reference)
            .sum::<u32>() as f64
            / graph.num_nodes as f64
    }
    pub fn stats(graph: &Graph) -> dto::Stadistics {
        let average_depth = average_depth(graph);
        let average_nodes_by_depth = average_nodes_by_depth(graph);
        let average_in_references = average_in_references(graph);
        dto::Stadistics {
            average_depth,
            average_nodes_by_depth,
            average_in_references,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::services::dto::Stadistics;
    use approx::*;

    const TEST: [(u32, u32, u32); 5] = [(1, 1, 0), (1, 2, 0), (2, 2, 1), (3, 3, 2), (3, 4, 3)];

    #[test]
    fn should_calculate_stats() {
        let graph = Graph::try_from(TEST.to_vec()).unwrap();
        let stats: Stadistics = stadistics::stats(&graph);
        assert_relative_eq!(1.33, stats.average_depth, epsilon = 0.01);
        assert_eq!(2.5, stats.average_nodes_by_depth);
        assert_relative_eq!(1.66, stats.average_in_references, epsilon = 0.01);
    }
}
