// Service module following DDD-clean architecture. It includes statistics function,
// the only service functions for the exercise.

/// Data transfer objects
pub mod dto {
    /// Statistics structure for displaying metrics data.
    #[derive(Debug)]
    pub struct Statistics {
        pub average_depth: f64,
        pub average_nodes_by_depth: f64,
        pub average_in_references: f64,
        pub last_transaction: u32,
        pub most_referenced_transaction: u32,
        pub range_timestamps: Vec<(u32, u64)>,
    }
}

/// Statistics services
pub mod statistics {
    use super::dto;
    use crate::graph::Graph;
    use std::collections::HashMap;
    pub const TIMESTAMP_RANGE: u32 = 10;

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

    // Iterate across all the nodes for setting up a ranking of timestamp.
    // This ranking can be precalculated in the load graph function if
    // it is necessary speedup it
    fn range_timestamps(graph: &Graph) -> Vec<(u32, u64)> {
        let mut range_timestamps: HashMap<u32, u64> = HashMap::new();
        for node in graph.nodes.values() {
            let range = node.timestamp / TIMESTAMP_RANGE;
            let entry = range_timestamps.entry(range).or_insert(0);
            *entry += 1;
        }
        let mut items = range_timestamps
            .iter()
            .map(|(&a, &b)| (a, b))
            .collect::<Vec<(u32, u64)>>();
        items.sort_by_key(|&k| k);
        items
    }

    /// Calculate statistics from graph `graph`.
    pub fn stats(graph: &Graph) -> dto::Statistics {
        let average_depth = average_depth(graph);
        let average_nodes_by_depth = average_nodes_by_depth(graph);
        let average_in_references = average_in_references(graph);
        let range_timestamps = range_timestamps(graph);
        let last_transaction = graph.metrics.last_transaction;
        let most_referenced_transaction = graph.metrics.most_in_reference_transaction;
        dto::Statistics {
            average_depth,
            average_nodes_by_depth,
            average_in_references,
            last_transaction,
            most_referenced_transaction,
            range_timestamps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::services::dto::Statistics;
    use approx::*;

    const TEST: [(u32, u32, u32); 5] = [(1, 1, 0), (1, 2, 0), (2, 2, 1), (3, 3, 2), (3, 4, 3)];

    const TEST_2: [(u32, u32, u32); 4] = [(1, 1, 0), (2, 2, 0), (3, 3, 1), (4, 4, 2)];

    const TEST_3: [(u32, u32, u32); 8] = [
        (1, 1, 0),
        (2, 2, 5),
        (3, 3, 9),
        (4, 4, 12),
        (1, 1, 22),
        (2, 2, 14),
        (3, 3, 41),
        (4, 4, 28),
    ];

    #[test]
    fn should_calculate_stats_test() {
        let graph = Graph::try_from(TEST.to_vec()).unwrap();
        let stats: Statistics = statistics::stats(&graph);
        assert_relative_eq!(1.33, stats.average_depth, epsilon = 0.01);
        assert_eq!(2.5, stats.average_nodes_by_depth);
        assert_relative_eq!(1.66, stats.average_in_references, epsilon = 0.01);
        assert_eq!(6, stats.last_transaction);
        assert_eq!(1, stats.most_referenced_transaction);
    }

    #[test]
    fn should_calculate_stats_test_2() {
        let graph = Graph::try_from(TEST_2.to_vec()).unwrap();
        let stats: Statistics = statistics::stats(&graph);
        assert_eq!(2.0, stats.average_depth);
        assert_eq!(1.0, stats.average_nodes_by_depth);
        assert_eq!(1.6, stats.average_in_references);
        assert_eq!(5, stats.last_transaction);
        assert_eq!(1, stats.most_referenced_transaction);
    }

    #[test]
    fn should_calculate_stats_timestamp() {
        let graph = Graph::try_from(TEST_3.to_vec()).unwrap();
        let range_timestamps: Vec<(u32, u64)> = statistics::stats(&graph).range_timestamps;
        assert_eq!(
            range_timestamps,
            vec![
                (0 as u32, 4 as u64),
                (1 as u32, 2 as u64),
                (2 as u32, 2 as u64),
                (4 as u32, 1 as u64)
            ]
        );
    }
}
