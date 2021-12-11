use std::iter::FusedIterator;
use std::mem;
use std::sync::Arc;

use super::node::Node;

pub struct NodeIterator<T:Copy> {
    children: Arc<Vec<Arc<Node<T>>>>,
    child_index: usize,
    parent: Option<Box<NodeIterator<T>>>,
}

impl<T:Copy> NodeIterator<T> {
    pub fn new(children: Arc<Vec<Arc<Node<T>>>>) -> Self {
        Self {
            children,
            child_index: 0,
            parent: None,
        }
    }
}

impl<T:Copy> core::iter::Iterator for NodeIterator<T> {
    type Item = Arc<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_child = {
            if self.child_index < self.children.len() {
                self.child_index += 1;
                Some(self.children[self.child_index].clone())
            } else {
                None
            }
        };

        match next_child {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                },
                None => None,
            },
            Some(node) => {
                *self = NodeIterator {
                    children: node.children.clone(),
                    child_index: 0,
                    parent: Some(Box::new(mem::take(self))),
                };
                Some(node.clone())
            },
        }
    }
}

impl<T:Copy> Default for NodeIterator<T> {
    fn default() -> Self {
        NodeIterator { children: Arc::new(Vec::new()), child_index: 0, parent: None }
    }
}

impl<T:Copy> FusedIterator for NodeIterator<T> {
}
