use grph::UndirectedGraph;
use std::{fs, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let edge_list = fs::read_to_string("data/Graph_ganzgross.txt")?;
    let graph = UndirectedGraph::from_edge_list(&edge_list)?;

    let now = Instant::now();
    let (counter, _markers) = graph.breadth_search_connected_components();
    let elapsed = now.elapsed();

    println!("Counter: {counter} in {:?} seconds.", elapsed);

    Ok(())
}
