//! To increase the performance of any queries into the graph we maintain an in memory
//! represntation of the graph in the form of an adjacency list.

use std::{path::PathBuf, collections::HashMap};

/// In memory representation of the graph. This maps every node path to an numerical id and
/// then build a graph using the ids.
pub struct NodeGraph {
    // currently free id
    free_id: usize,
    paths: Vec<PathBuf>,
    path_to_id: HashMap<PathBuf, usize>,
    adjacency_matrix: AdjacencyMatrix,
    adjacency_list: AdjacencyList,
}

struct AdjacencyMatrix {

}

struct AdjacencyList {
    n_nodes: usize,
    list: Vec<Vec<usize>>
}

impl AdjacencyList {
    fn new(n_nodes: usize) -> Self {
        let vec = Vec::new();
        let list = vec![vec; n_nodes];
        Self {
            n_nodes,
            list,
        }
    }
}

