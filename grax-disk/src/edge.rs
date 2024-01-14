use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use cap_std::{ambient_authority, fs::Dir};
use grax_algorithms::dfs_sp;
use grax_core::{graph::MutGraph, index::NodeId};
use serde::{Deserialize, Serialize};

use crate::error::DiskGraphResult;

pub trait EdgeDestination {
    fn destination(&self) -> PathBuf;
}

#[derive(Debug)]
pub struct EdgeDiskGraph<G>
where
    G: MutGraph + Serialize + for<'de> Deserialize<'de>,
    G::EdgeWeight: EdgeDestination,
{
    pub graph: G,
    dir: Dir,
}

impl<G> EdgeDiskGraph<G>
where
    G: MutGraph + Serialize + for<'de> Deserialize<'de>,
    G::EdgeWeight: EdgeDestination,
    G::Key: 'static,
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

        Ok(Self { dir, graph })
    }

    pub fn read(
        &self,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        buf: &mut Vec<u8>,
    ) -> std::io::Result<usize> {
        if let Some(dest) = self.find_destination(from, to) {
            let mut file = self.dir.open(dest)?;
            file.read(buf)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Node does not exist",
            ))
        }
    }

    pub fn write(
        &self,
        from: NodeId<G::Key>,
        to: NodeId<G::Key>,
        buf: &[u8],
    ) -> std::io::Result<usize> {
        if let Some(dest) = self.find_destination(from, to) {
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
        let parents = dfs_sp(&self.graph, source, sink, |_| true)?;
        let mut path_vec = parents
            .iter_parent_edges(source, sink)
            .map(|edge_id| {
                let edge = self.graph.edge(edge_id).unwrap();
                edge.weight.destination()
            })
            .collect::<Vec<_>>();
        path_vec.reverse();
        let path = path_vec.into_iter().collect::<PathBuf>();
        Some(path)
    }
}
