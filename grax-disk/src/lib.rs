use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use cap_std::{
    ambient_authority,
    fs::{Dir, File, OpenOptions},
};
use grax_core::{
    collections::{FixedNodeMap, VisitNodeMap},
    edge::EdgeRef,
    graph::MutGraph,
    index::NodeId,
};
use serde::{Deserialize, Serialize};

use error::*;
use node::*;

pub mod error;
pub mod node;

#[derive(Debug)]
pub struct DiskGraph<G>
where
    G: MutGraph + Serialize + for<'de> Deserialize<'de>,
    G::NodeWeight: NodeDestination,
{
    pub graph: G,
    dir: Dir,
    // cache: G::NodeMap<Option<File>>,
}

impl<G> DiskGraph<G>
where
    G: MutGraph + Serialize + for<'de> Deserialize<'de>,
    G::NodeWeight: NodeDestination,
{
    pub fn create<P: AsRef<Path>>(path: P) -> DiskGraphResult<Self> {
        fs::create_dir_all(&path)?;
        let dir = Dir::open_ambient_dir(path, ambient_authority())?;
        let graph = G::new();

        let buf = serde_json::to_vec(&graph)?;
        let mut graph_file = dir.create("graph.json")?;
        graph_file.write_all(&buf)?;

        Ok(Self { dir, graph })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> DiskGraphResult<Self> {
        let dir = Dir::open_ambient_dir(path, ambient_authority())?;
        let graph_file = dir.open("graph.json")?;
        let graph: G = serde_json::from_reader(graph_file)?;
        // let cache = graph.node_map();

        Ok(Self { dir, graph })
    }

    pub fn read(&self, node_id: NodeId<G::Key>, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        // if let Some(file) = self.cache.get_mut(node_id) {
        //     file.read(buf)
        // } else

        if let Some(node) = self.graph.node(node_id) {
            let dest = node.weight.destination();
            let mut file = self.dir.open(dest)?;
            file.read(buf)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Node does not exist",
            ))
        }
    }

    pub fn write(&self, node_id: NodeId<G::Key>, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(node) = self.graph.node(node_id) {
            let dest = node.weight.destination();
            let parent = dest.parent().unwrap();
            self.dir.create_dir_all(parent)?;
            let mut file = self.dir.create(dest)?;
            file.write(buf)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Node does not exist",
            ))
        }
    }

    pub fn commit(&self) -> std::io::Result<()> {
        let contents = serde_json::to_vec(&self.graph)?;
        self.dir.write("graph.json", &contents)
    }

    // TODO use EdgeWeight for paths associated to then compute destination paths by looking at the edges between two nodes
    fn find_destination(&self, source: NodeId<G::Key>, sink: NodeId<G::Key>) -> Option<PathBuf> {
        let mut stack = Vec::new();
        let mut visited = self.graph.visit_node_map();
        let mut paths = self.graph.fixed_node_map(PathBuf::new());

        stack.push(source);
        visited.visit(source);

        while let Some(from) = stack.pop() {
            if from == sink {
                return Some(paths.get(sink).to_owned());
            }

            for EdgeRef { edge_id, weight } in self.graph.iter_adjacent_edges(from) {
                let to = edge_id.to();
                if !visited.is_visited(to) {
                    stack.push(to);
                    visited.visit(to);
                }
            }
        }
        None
    }
}
