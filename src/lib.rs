use std::mem;
use std::iter::FusedIterator;
use slotmap::{
    SlotMap,
    new_key_type,
};

new_key_type! { pub struct NodeKey; }

pub struct Scenegraph {
    root_key: NodeKey,
    slot_map: SlotMap<NodeKey, Node>,
}

impl Scenegraph {
    pub fn new() -> Self {
        let mut sm = SlotMap::with_key();
        let root_key = sm.insert(Node::new());

        Scenegraph { 
            root_key: root_key,
            slot_map: sm,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut sm = SlotMap::with_capacity_and_key(capacity);
        let root_key = sm.insert(Node::new());

        Scenegraph { 
            root_key: root_key,
            slot_map: sm,
        }
    }

    pub fn insert_node(&mut self, node: Node) -> NodeKey {
        self.slot_map.insert(node)
    }

    pub fn get_node(&self, key: NodeKey) -> Option<&Node> {
        self.slot_map.get(key)
    }

    pub fn get_node_mut(&mut self, key: NodeKey) -> Option<&mut Node> {
        self.slot_map.get_mut(key)
    }

    pub fn iter_from_node<'a>(&self, from: NodeKey) -> NodeIterator<'_> {
        let from_node = self.get_node(from).unwrap();
        NodeIterator {
            child_keys: from_node.child_keys.as_slice(),
            parent: None,
            scene_graph: self,
        }
    }

    pub fn iter(&self) -> NodeIterator<'_> {
        self.iter_from_node(self.root_key)
    }
}

pub struct Node {
    child_keys: Vec<NodeKey>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            child_keys: Vec::new(),
        }
    }
}

/// A Node iterator - based on https://aloso.github.io/2021/03/09/creating-an-iterator
pub struct NodeIterator<'a> {
    child_keys: &'a [NodeKey],
    parent: Option<Box<NodeIterator<'a>>>,
    scene_graph: &'a Scenegraph,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.child_keys.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(node_key) => {
                self.child_keys = &self.child_keys[1..];

                // start iterating the child trees
                let node = self.scene_graph.get_node(*node_key).unwrap();
                let empty_iterator = NodeIterator {
                    child_keys: &[], 
                    parent: None,
                    scene_graph: self.scene_graph,
                };

                *self = NodeIterator {
                    child_keys: node.child_keys.as_slice(),
                    parent: Some(Box::new(mem::replace(self, empty_iterator))),
                    scene_graph: self.scene_graph,
                };
                self.next()
            }
        }
    }
}

impl FusedIterator for NodeIterator<'_> {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
