use super::path::PrefixedPath;
use crate::crypto::hash::Digest;
use super::Syncable;
use super::Node;

// Data structure used to synchronize two SyncSets
pub enum Set<Data> {
    // Lightweight alternative, only contains the hash of 
    // the sub-tree at prefix
    LabelSet {
        prefix: PrefixedPath,
        label: Digest
    },

    // Heavy alternative, contains all the data of a sub-tree at
    // a given prefix
    DataSet {
        underlying: Vec<Data>,
        prefix: PrefixedPath,
        dump: bool
    }
}

impl<Data: Syncable> Set<Data> {

    // Constructors, for ease of use
    pub(super) fn new_dataset(prefix: PrefixedPath, node: &Node<Data>, dump: bool) -> Set<Data> {
        let mut underlying: Vec<Data> = Vec::with_capacity(node.size());
        node.traverse(&mut |elem| underlying.push(elem.clone()));

        Set::DataSet{underlying, prefix, dump}
    }

    pub(super) fn new_empty_dataset(prefix: PrefixedPath, dump: bool) -> Set<Data> {
        let underlying = Vec::new();
        Set::DataSet{underlying, prefix, dump}
    }

}