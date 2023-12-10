// Domain entities. It includes the transaction and metrics structures.
// !The main graph can be considered a domain entity, but in this case, it has too much responsabilities and
// for these reasons it was moved in another module `graph.rs`
use std::fmt;

/// Structure for the transaction - node representation
/// it includes `id` `timestamp` both `parents` and its metrics
/// `TransactionMetrics`.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: u32,
    pub timestamp: u32,
    pub parents: Option<(u32, u32)>,
    pub metrics: TransactionMetrics,
}

/// Transaction metrics
///
/// - `depth` for a transaction  from the root node
/// - `in_reference` current in references
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransactionMetrics {
    pub depth: u32,
    pub in_reference: u32,
}
/// Structure for saving graph metrics.
///
/// - `last_transaction` identifier from last transaction in time (last timestamp).
/// - `most_in_reference_transaction` identifier for node transaction with most in references.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GeneralMetrics {
    pub last_transaction: u32,
    pub most_in_reference_transaction: u32,
}

impl fmt::Display for TransactionMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!(
            "(depth={:},in_reference={:})",
            self.depth, self.in_reference
        );
        write!(f, "{}", output)
    }
}

impl fmt::Display for GeneralMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!(
            "(last_transaction={:},most_referenced_transaction={:})",
            self.last_transaction, self.most_in_reference_transaction
        );
        write!(f, "{}", output)
    }
}

impl Transaction {
    /// Constructor for a transaction. It has an unique id `id` with its parents `left_parent` and
    /// `right_parent` and its timestamp `timestamp`
    pub fn new(id: u32, left_parent: u32, right_parent: u32, timestamp: u32) -> Self {
        Transaction {
            id,
            timestamp,
            parents: Some((left_parent, right_parent)),
            metrics: Default::default(),
        }
    }
}

impl fmt::Display for Transaction {
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
