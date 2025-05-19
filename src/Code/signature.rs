use crate::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

pub fn truss_decomposition<N, E: core::fmt::Debug>(
    graph: UnGraph<N, E>,
    h: usize,
) -> HashMap<(NodeIndex, NodeIndex), usize> { ... }