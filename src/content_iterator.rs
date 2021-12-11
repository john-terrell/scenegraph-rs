use std::iter::FusedIterator;
use std::mem;
use std::sync::Arc;

use super::node::Node;

pub struct ContentIterator<T:Copy> {
    children: Arc<Vec<Arc<Node<T>>>>,
    child_index: usize,
    parent: Option<Box<ContentIterator<T>>>,
}

impl<T:Copy> ContentIterator<T> {
    pub fn new(children: Arc<Vec<Arc<Node<T>>>>) -> Self {
        Self {
            children,
            child_index: 0,
            parent: None,
        }
    }
}

impl<T:Copy> core::iter::Iterator for ContentIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_child = {
            if self.child_index < self.children.len() {
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
                self.child_index += 1;

                *self = ContentIterator {
                    children: node.children.clone(),
                    child_index: 0,
                    parent: Some(Box::new(mem::take(self))),
                };
                Some(node.contents)
            },
        }
    }
}

impl<T:Copy> Default for ContentIterator<T> {
    fn default() -> Self {
        ContentIterator { children: Arc::new(Vec::new()), child_index: 0, parent: None }
    }
}

impl<T:Copy> FusedIterator for ContentIterator<T> {
}

#[cfg(test)]
mod tests {
    use super::{Node};
    use std::sync::Arc;

    #[test]
    fn iteration() {
        let tree = Arc::new(
            Node::new(0, vec![
                Arc::new(Node::new(1, vec![])),
                Arc::new(Node::new(2, vec![])),
                Arc::new(Node::new(3, vec![])),
                Arc::new(Node::new(4, vec![
                    Arc::new(Node::new(40, vec![])),
                    Arc::new(Node::new(41, vec![])),
                    Arc::new(Node::new(42, vec![
                        Arc::new(Node::new(420, vec![])),
                        Arc::new(Node::new(421, vec![])),    
                    ])),
                ])),
                Arc::new(Node::new(5, vec![])),
                Arc::new(Node::new(6, vec![
                    Arc::new(Node::new(60, vec![])),
                    Arc::new(Node::new(61, vec![])),
                    Arc::new(Node::new(62, vec![
                        Arc::new(Node::new(620, vec![])),
                        Arc::new(Node::new(621, vec![])),    
                    ])),
                ])),
                Arc::new(Node::new(7, vec![])),
            ])
        );

        let nums: Vec<i32> = tree.content_iter().collect();
        assert_eq!(nums, vec![0, 1, 2, 3, 4, 40, 41, 42, 420, 421, 5, 6, 60, 61, 62, 620, 621, 7]);
    }

    #[test]
    fn multithreaded_iteration() {
        let tree = Arc::new(
            Node::new(0, vec![
                Arc::new(Node::new(1, vec![])),
                Arc::new(Node::new(2, vec![])),
                Arc::new(Node::new(3, vec![])),
                Arc::new(Node::new(4, vec![
                    Arc::new(Node::new(40, vec![])),
                    Arc::new(Node::new(41, vec![])),
                    Arc::new(Node::new(42, vec![
                        Arc::new(Node::new(420, vec![])),
                        Arc::new(Node::new(421, vec![])),    
                    ])),
                ])),
                Arc::new(Node::new(5, vec![])),
                Arc::new(Node::new(6, vec![
                    Arc::new(Node::new(60, vec![])),
                    Arc::new(Node::new(61, vec![])),
                    Arc::new(Node::new(62, vec![
                        Arc::new(Node::new(620, vec![])),
                        Arc::new(Node::new(621, vec![])),    
                    ])),
                ])),
                Arc::new(Node::new(7, vec![])),
            ])
        );

        let mut handles = vec![];
        for _ in 0..10 {
            let tree = Arc::clone(&tree);
            let handle = std::thread::spawn(move || {
                let nums: Vec<i32> = tree.content_iter().collect();
                assert_eq!(nums, vec![0, 1, 2, 3, 4, 40, 41, 42, 420, 421, 5, 6, 60, 61, 62, 620, 621, 7]);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
