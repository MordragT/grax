use grph::{
    deser::{EdgeList, EdgeListOptions},
    UndirectedAdjGraph,
};
use std::{fs, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let edge_list = fs::read_to_string("data/Graph_ganzgross.txt")?;
    let edge_list = EdgeList::new(&edge_list, EdgeListOptions { weighted: false });
    let graph = UndirectedAdjGraph::<usize, ()>::try_from(edge_list).unwrap();

    let now = Instant::now();
    let (counter, _markers) = graph.breadth_search_connected_components();
    let elapsed = now.elapsed();

    println!("Counter: {counter} in {:?}", elapsed);

    Ok(())
}
