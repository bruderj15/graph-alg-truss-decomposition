for k in 2..=upper_trussness {
    while !bin.get(&k).unwrap_or(&HashSet::new()).is_empty() {
        let uv @ (u, v) = pick(bin.get_mut(&k).unwrap()).unwrap();
        let e = graph.find_edge(u, v).unwrap();
        trussness.insert(uv, k);

        graph.remove_edge(e);
        let u_affected_edges = h_hop_edges_node(&graph, &u, h);
        let v_affected_edges = h_hop_edges_node(&graph, &v, h);
        for _e in u_affected_edges.union(&v_affected_edges).copied() {
            let _uv @ (_u, _v) = graph.edge_endpoints(_e).unwrap();
            let old_edge_upper_trussness = sup.get(&_uv).unwrap() + 2;
            if let Some(old_trash) = bin.get_mut(&old_edge_upper_trussness) {
                old_trash.remove(&_uv);
            }
            let new_sup = h_support_edge(&graph, _e, h);
            sup.insert(_uv, new_sup);
            bin.entry(max(new_sup + 2, k))
                .or_insert(HashSet::new())
                .insert(_uv);
        }
    }
}
trussness