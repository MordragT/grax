extern crate test;

use crate::prelude::*;
use more_asserts::*;
use std::{fs, str::FromStr};
use test::Bencher;

#[test]
fn add_node() {
    let mut graph = AdjacencyList::<u32, ()>::new();
    let _idx1 = graph.add_node(1);
    let _idx2 = graph.add_node(2);
    let _idx3 = graph.add_node(3);

    graph.contains_node(&1).unwrap();
    graph.contains_node(&2).unwrap();
    graph.contains_node(&3).unwrap();

    assert!(graph.contains_node(&100).is_none());
}

#[test]
fn update_node() {
    let mut graph = AdjacencyList::<u32, ()>::new();
    let idx1 = graph.add_node(1);

    assert_eq!(graph.update_node(idx1, 5), 1);

    graph.contains_node(&5).unwrap();
    assert!(graph.contains_node(&1).is_none());
}

#[test]
fn add_edge() {
    let mut graph = AdjacencyList::<u32, ()>::new();
    let idx1 = graph.add_node(1);
    let idx2 = graph.add_node(2);
    let _idx3 = graph.add_node(3);

    let _ = graph.add_edge(idx1, idx2, ()).unwrap();

    graph.contains_edge(idx1, idx2).unwrap();
    //graph.contains_edge(idx2, idx1).unwrap();

    assert!(graph.contains_edge(idx2, idx1).is_none());
}

#[test]
fn update_edge() {
    let mut graph = AdjacencyList::<u32, u32>::new();
    let idx1 = graph.add_node(1);
    let idx2 = graph.add_node(2);

    let edge = graph.add_edge(idx1, idx2, 2).unwrap();

    assert_eq!(graph.update_edge(edge, 5), 2);
    assert_eq!(graph.weight(edge), &5);
}

#[test]
fn from_edge_list() {
    let edge_list = "4
        0 2
        1 2
        2 3
        3 1";
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    assert_eq!(graph.node_count(), 4);

    let idx0 = graph.contains_node(&0).unwrap();
    let idx1 = graph.contains_node(&1).unwrap();
    let idx2 = graph.contains_node(&2).unwrap();
    let idx3 = graph.contains_node(&3).unwrap();

    graph.contains_edge(idx0, idx2).unwrap();
    graph.contains_edge(idx1, idx2).unwrap();
    graph.contains_edge(idx2, idx3).unwrap();
    graph.contains_edge(idx3, idx1).unwrap();

    graph.contains_edge(idx2, idx0).unwrap();
    graph.contains_edge(idx2, idx1).unwrap();
    graph.contains_edge(idx3, idx2).unwrap();
    graph.contains_edge(idx1, idx3).unwrap();

    assert!(graph.contains_edge(idx1, idx0).is_none());
}

#[test]
fn djikstra() {
    let edge_list = EdgeList::with(
        [
            (0, 1, 1.0),
            (0, 2, 3.0),
            (1, 2, 1.0),
            (2, 3, 4.0),
            (3, 0, 1.5),
        ]
        .into_iter(),
        4,
    );

    let graph = AdjacencyList::<usize, f32>::from_edge_list(edge_list, false).unwrap();
    let dist = graph.dijkstra(NodeIndex(0), NodeIndex(2));

    assert_eq!(dist, Some(2.0));
}

#[test]
fn kruskal_mst() {
    let edge_list = EdgeList::with(
        [
            (0, 1, 1.0),
            (0, 2, 3.0),
            (1, 2, 1.0),
            (2, 3, 4.0),
            (3, 0, 1.5),
        ]
        .into_iter(),
        4,
    );

    let graph = AdjacencyList::<usize, f32>::from_edge_list(edge_list, false).unwrap();
    let mst = graph.kruskal_mst();

    dbg!(mst);

    assert!(false);
}

#[bench]
fn breadth_search_connected_components_graph1(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph1.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.breadth_search_connected_components();
        assert_eq!(counter, 2);
    });
}

#[bench]
fn breadth_search_connected_components_graph2(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.breadth_search_connected_components();
        assert_eq!(counter, 4);
    });
}

#[bench]
fn breadth_search_connected_components_graph3(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.breadth_search_connected_components();
        assert_eq!(counter, 4);
    });
}

#[bench]
fn breadth_search_connected_components_graph_gross(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.breadth_search_connected_components();
        assert_eq!(counter, 222);
    });
}

#[bench]
fn breadth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.breadth_search_connected_components();
        assert_eq!(counter, 9560);
    });
}

#[bench]
fn breadth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.breadth_search_connected_components();
        assert_eq!(counter, 306);
    });
}

#[bench]
fn depth_search_connected_components_graph1(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph1.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.depth_search_connected_components();
        assert_eq!(counter, 2);
    });
}

#[bench]
fn depth_search_connected_components_graph2(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph2.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.depth_search_connected_components();
        assert_eq!(counter, 4);
    });
}

#[bench]
fn depth_search_connected_components_graph3(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph3.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.depth_search_connected_components();
        assert_eq!(counter, 4);
    });
}

#[bench]
fn depth_search_connected_components_graph_gross(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph_gross.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.depth_search_connected_components();
        assert_eq!(counter, 222);
    });
}

#[bench]
fn depth_search_connected_components_graph_ganz_gross(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph_ganzgross.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.depth_search_connected_components();
        assert_eq!(counter, 9560);
    });
}

#[bench]
fn depth_search_connected_components_graph_ganz_ganz_gross(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/Graph_ganzganzgross.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let counter = graph.depth_search_connected_components();
        assert_eq!(counter, 306);
    });
}

#[bench]
fn prim_graph_1_2(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_1_2.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.prim() as f32;
        assert_eq!(count, 287.32286);
    })
}

#[bench]
fn prim_graph_1_20(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_1_20.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.prim() as f32;
        assert_eq!(count, 36.86275);
    })
}

#[bench]
fn prim_graph_1_200(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.prim() as f32;
        assert_eq!(count, 12.68182);
    })
}

#[bench]
fn prim_graph_10_20(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_10_20.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.prim() as f32;
        assert_eq!(count, 2785.62417);
    })
}

#[bench]
fn prim_graph_10_200(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_10_200.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.prim() as f32;
        assert_eq!(count, 372.14417);
    })
}

#[bench]
fn prim_graph_100_200(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_100_200.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.prim() as f32;
        assert_eq!(count, 27550.51488);
    })
}

#[bench]
fn kruskal_graph_1_2(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_1_2.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.kruskal_weight() as f32;
        assert_eq!(count, 287.32286);
    })
}

#[bench]
fn kruskal_graph_1_20(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_1_20.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.kruskal_weight() as f32;
        assert_eq!(count, 36.86275);
    })
}

#[bench]
fn kruskal_graph_1_200(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_1_200.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.kruskal_weight() as f32;
        assert_eq!(count, 12.68182);
    })
}

#[bench]
fn kruskal_graph_10_20(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_10_20.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.kruskal_weight() as f32;
        assert_eq!(count, 2785.62417);
    })
}

#[bench]
fn kruskal_graph_10_200(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_10_200.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.kruskal_weight() as f32;
        assert_eq!(count, 372.14417);
    })
}

#[bench]
fn kruskal_graph_100_200(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/G_100_200.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let count = graph.kruskal_weight() as f32;
        assert_eq!(count, 27550.51488);
    })
}

#[bench]
fn nearest_neighbor_k_10(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.nearest_neighbor_from_first().unwrap().weight;
        assert_le!(total, 38.41 * 1.2);
    })
}

#[bench]
fn nearest_neighbor_k_10e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.nearest_neighbor_from_first().unwrap().weight;
        assert_le!(total, 27.26 * 1.2);
    })
}

#[bench]
fn nearest_neighbor_k_12(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.nearest_neighbor_from_first().unwrap().weight;
        assert_le!(total, 45.19 * 1.2);
    })
}

#[bench]
fn nearest_neighbor_k_12e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.nearest_neighbor_from_first().unwrap().weight;
        assert_le!(total, 36.13 * 1.2);
    })
}

#[bench]
fn double_tree_k_10(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.double_tree().unwrap().weight;
        assert_le!(total, 38.41 * 1.2);
    })
}

#[bench]
fn double_tree_k_10e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.double_tree().unwrap().weight;
        assert_le!(total, 27.26 * 1.2);
    })
}

#[bench]
fn double_tree_k_12(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.double_tree().unwrap().weight;
        assert_le!(total, 45.19 * 1.2);
    })
}

#[bench]
fn double_tree_k_12e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.double_tree().unwrap().weight;
        assert_le!(total, 36.13 * 1.2);
    })
}

#[bench]
fn branch_bound_k_10(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound().unwrap().weight as f32;
        assert_eq!(total, 38.41);
    })
}

#[bench]
fn branch_bound_k_10e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound().unwrap().weight as f32;
        assert_eq!(total, 27.26);
    })
}

#[bench]
fn branch_bound_k_12(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound().unwrap().weight as f32;
        assert_eq!(total, 45.19);
    })
}

#[bench]
fn branch_bound_k_12e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound().unwrap().weight as f32;
        assert_eq!(total, 36.13);
    })
}

#[bench]
fn branch_bound_rec_k_10(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound_rec().unwrap().weight as f32;
        assert_eq!(total, 38.41);
    })
}

#[bench]
fn branch_bound_rec_k_10e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_10e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound_rec().unwrap().weight as f32;
        assert_eq!(total, 27.26);
    })
}

#[bench]
fn branch_bound_rec_k_12(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound_rec().unwrap().weight as f32;
        assert_eq!(total, 45.19);
    })
}

#[bench]
fn branch_bound_rec_k_12e(b: &mut Bencher) {
    let edge_list = fs::read_to_string("data/K_12e.txt").unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    b.iter(|| {
        let total = graph.branch_bound_rec().unwrap().weight as f32;
        assert_eq!(total, 36.13);
    })
}
