let mut graph = graph;
let mut bin = HashMap::<usize, HashSet<(NodeIndex, NodeIndex)>>::new();
let mut upper_trussness = 2;
let mut trussness = HashMap::<usize, (NodeIndex, NodeIndex)>::new();
let mut sup: HashMap<(NodeIndex, NodeIndex), usize> = h_support(&graph, h);
for (e, sup) in sup.iter() {
    let edge_upper_trussness = sup + 2;
    bin.entry(edge_upper_trussness)
        .or_insert(HashSet::new())
        .insert(*e);
    upper_trussness = max(upper_trussness, edge_upper_trussness);
}