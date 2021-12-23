use super::Tile;

pub type ClusterId = String;
pub type ClusterIdRef = str;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cluster {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}
