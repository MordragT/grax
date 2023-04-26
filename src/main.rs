use grph::{edge_list::EdgeList, graph::UndirectedAdjGraph};
use std::{fs, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prim();
    Ok(())
}

fn graph_gross() -> Result<(), Box<dyn std::error::Error>> {
    let edge_list = fs::read_to_string("data/Graph_ganzgross.txt")?;
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

    let now = Instant::now();
    let (counter, _markers) = graph.breadth_search_connected_components();
    let elapsed = now.elapsed();

    println!("Counter: {counter} in {:?}", elapsed);

    Ok(())
}

fn prim() {
    let edge_list = r#"7
    1 2 0.2
    1 3 0.3
    1 4 0.3
    2 3 0.4
    2 5 0.3
    3 4 0.5
    3 5 0.1
    4 6 0.7
    5 6 0.8
    6 0 0.9"#;
    let edge_list = EdgeList::from_str(&edge_list).unwrap();
    let graph = UndirectedAdjGraph::<usize, f64>::try_from(edge_list).unwrap();
    let total = graph.prim();

    assert_eq!(total, 2.5);
}
