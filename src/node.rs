use std::iter::FusedIterator;
use std::mem;
use std::sync::{
    Arc,
};

// see https://aloso.github.io/2021/03/09/creating-an-iterator
pub struct Node<T> {
    contents: T,
    children: Arc<Vec<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(contents: T, children: Vec<Node<T>>) -> Self {
        Self {
            contents,
            children: Arc::new(children),
        }
    }
}

impl<T> Node<T> {
    fn iter(&self) -> Iterator<'_, T> {
        Iterator {
            children: std::slice::from_ref(self),
            parent: None,
        }
    }
}

pub struct Iterator<'a, T> {
    children: &'a [Node<T>],
    parent: Option<Box<Iterator<'a, T>>>,
}

impl<'a, T> core::iter::Iterator for Iterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(node) => {
                self.children = &self.children[1..];

                *self = Iterator {
                    children: node.children.as_slice(),
                    parent: Some(Box::new(mem::take(self))),
                };
                Some(&node.contents)
            },
        }
    }
}

impl<'a, T> Default for Iterator<'_, T> {
    fn default() -> Self {
        Iterator { children: &[], parent: None }
    }
}

impl<'a, T> FusedIterator for Iterator<'a, T> {
}

#[cfg(test)]
mod tests {
    use super::{Node};
    use std::sync::Arc;

    #[test]
    fn iteration() {
        let tree = Node::new(0, vec![
            Node::new(1, vec![]),
            Node::new(2, vec![]),
            Node::new(3, vec![]),
            Node::new(4, vec![
                Node::new(40, vec![]),
                Node::new(41, vec![]),
                Node::new(42, vec![
                    Node::new(420, vec![]),
                    Node::new(421, vec![]),    
                ]),
            ]),
            Node::new(5, vec![]),
            Node::new(6, vec![
                Node::new(60, vec![]),
                Node::new(61, vec![]),
                Node::new(62, vec![
                    Node::new(620, vec![]),
                    Node::new(621, vec![]),    
                ]),
            ]),
            Node::new(7, vec![]),
        ]);

        let nums: Vec<i32> = tree.iter().copied().collect();
        assert_eq!(nums, vec![0, 1, 2, 3, 4, 40, 41, 42, 420, 421, 5, 6, 60, 61, 62, 620, 621, 7]);
    }

    #[test]
    fn multithreaded_iteration() {
        let tree = Arc::new(Node::new(0, vec![
                Node::new(1, vec![]),
                Node::new(2, vec![]),
                Node::new(3, vec![]),
                Node::new(4, vec![
                    Node::new(40, vec![]),
                    Node::new(41, vec![]),
                    Node::new(42, vec![
                        Node::new(420, vec![]),
                        Node::new(421, vec![]),    
                    ]),
                ]),
                Node::new(5, vec![]),
                Node::new(6, vec![
                    Node::new(60, vec![]),
                    Node::new(61, vec![]),
                    Node::new(62, vec![
                        Node::new(620, vec![]),
                        Node::new(621, vec![]),    
                    ]),
                ]),
                Node::new(7, vec![]),
            ])
        );

        let mut handles = vec![];
        for _ in 0..10 {
            let tree = Arc::clone(&tree);
            let handle = std::thread::spawn(move || {
                let nums: Vec<i32> = tree.iter().copied().collect();
                assert_eq!(nums, vec![0, 1, 2, 3, 4, 40, 41, 42, 420, 421, 5, 6, 60, 61, 62, 620, 621, 7]);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
