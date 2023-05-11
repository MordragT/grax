use grph::prelude::*;
use std::{fs, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    nearest_neighbor("data/K_10.txt", Some(38.41));
    // nearest_neighbor("data/K_10e.txt", Some(27.26));
    // nearest_neighbor("data/K_12.txt", Some(45.19));
    // nearest_neighbor("data/K_12e.txt", Some(36.133));
    // nearest_neighbor("data/K_15.txt", None);
    // nearest_neighbor("data/K_15e.txt", None);
    // nearest_neighbor("data/K_20.txt", None);
    // nearest_neighbor("data/K_30.txt", None);
    // nearest_neighbor("data/K_50.txt", None);
    // nearest_neighbor("data/K_70.txt", None);
    // nearest_neighbor("data/K_100.txt", None);

    double_tree("data/K_10.txt", Some(38.41));
    // double_tree("data/K_10e.txt", Some(27.26));
    // double_tree("data/K_12.txt", Some(45.19));
    // double_tree("data/K_12e.txt", Some(36.133));
    // double_tree("data/K_15.txt", None);
    // double_tree("data/K_15e.txt", None);
    // double_tree("data/K_20.txt", None);
    // double_tree("data/K_30.txt", None);
    // double_tree("data/K_50.txt", None);
    // double_tree("data/K_70.txt", None);
    // double_tree("data/K_100.txt", None);

    branch_bound("data/K_10.txt");

    // db();

    // depth_search("data/K_10.txt");

    Ok(())
}

pub fn branch_bound(path: &str) {
    let edge_list = fs::read_to_string(path).unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();
    let total = graph.branch_bound().unwrap() as f32;
    println!("bb: {path}: {total}");
}

pub fn nearest_neighbor(path: &str, optimal: Option<f32>) {
    let edge_list = fs::read_to_string(path).unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();
    let total = graph.nearest_neighbor().unwrap() as f32;
    match optimal {
        Some(opt) => println!("nn: {path} with {opt}: {total}"),
        None => println!("nn: {path}: {total}"),
    }
}

pub fn double_tree(path: &str, optimal: Option<f32>) {
    let edge_list = fs::read_to_string(path).unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let mut graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();
    let total = graph.double_tree().unwrap() as f32;
    match optimal {
        Some(opt) => println!("dt: {path} with {opt}: {total}"),
        None => println!("dt: {path}: {total}"),
    }
}

pub fn db() {
    let edge_list = EdgeList::with(
        [
            (0, 1, 1.0),
            (1, 2, 1.0),
            (2, 3, 1.0),
            (3, 0, 10.0),
            (0, 2, 2.0),
            (1, 3, 2.0),
        ]
        .into_iter(),
        4,
    );
    let mut graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();
    let total = graph.double_tree().unwrap();

    println!("{total}")
}

pub fn nn() {
    let edge_list = EdgeList::with(
        [
            (0, 1, 1.0),
            (1, 2, 1.0),
            (2, 3, 1.0),
            (3, 0, 10.0),
            (0, 2, 2.0),
            (1, 3, 2.0),
        ]
        .into_iter(),
        4,
    );
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();
    let nn = graph.nearest_neighbor().unwrap();

    println!("{nn}")
}

pub fn prim() {
    let edge_list = EdgeList::with(
        [
            (1, 2, 0.2),
            (1, 3, 0.3),
            (1, 4, 0.3),
            (2, 3, 0.4),
            (2, 5, 0.3),
            (3, 4, 0.5),
            (3, 5, 0.1),
            (4, 6, 0.7),
            (5, 6, 0.8),
            (6, 0, 0.9),
        ]
        .into_iter(),
        7,
    );
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();
    let total = graph.prim();

    assert_eq!(total, 2.5);
}

pub fn graph_gross() -> Result<(), Box<dyn std::error::Error>> {
    let edge_list = fs::read_to_string("data/Graph_ganzgross.txt")?;
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, ()>::from_edge_list(edge_list, false).unwrap();

    let now = Instant::now();
    let counter = graph.breadth_search_connected_components();
    let elapsed = now.elapsed();

    println!("Counter: {counter} in {:?}", elapsed);

    Ok(())
}

pub fn depth_search(path: &str) {
    let edge_list = fs::read_to_string(path).unwrap();
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = AdjacencyList::<usize, f64>::from_edge_list(edge_list, false).unwrap();

    let connected_components = graph.depth_search_connected_components();

    println!("Connected Components: {connected_components}");
}
