use std::sync::Arc;
use super::node_iterator::NodeIterator;
use super::content_iterator::ContentIterator;

// see https://aloso.github.io/2021/03/09/creating-an-iterator
#[derive(Debug, Clone)]
pub struct Node<T:Copy> {
    pub contents: T,
    pub children: Arc<Vec<Arc<Node<T>>>>,
}

impl<T:Copy> Node<T> {
    pub fn new(contents: T, children: Vec<Arc<Node<T>>>) -> Self {
        Self {
            contents,
            children: Arc::new(children),
        }
    }

    pub fn add_child(&mut self, child: Arc<Node<T>>) {
        // Adding a child uses make_mut to create a new
        // children vector allocation if someone else is currently holding
        // a reference (e.g. someone else is traversing while a child
        // is being added.
        let children = Arc::make_mut(&mut self.children);
        children.push(child);
    }
}

impl<T:Copy> Node<T> {
    pub fn iter_nodes(self: Arc<Node<T>>) -> NodeIterator<T> {
        let top: Arc<Vec<Arc<Node<T>>>> = Arc::new(vec![self.clone()]);
        NodeIterator::new(top)
    }

    pub fn iter_contents(self: Arc<Node<T>>) -> ContentIterator<T> {
        let top: Arc<Vec<Arc<Node<T>>>> = Arc::new(vec![self.clone()]);
        ContentIterator::new(top)
    }
}
