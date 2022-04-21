use std::collections::VecDeque;

pub type OptionNodeRef<T> = Option<Box<Node<T>>>;
pub type NodeRef<T> = Box<Node<T>>;

#[derive(Debug, Default, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub left: OptionNodeRef<T>,
    pub right: OptionNodeRef<T>,
}

impl From<i32> for Node<i32> {
    fn from(val: i32) -> Self {
        Node::<i32>::new(val)
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node::<T> {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left<N>(mut self, node: N) -> Self
    where
        N: Into<Node<T>>,
        T: std::fmt::Debug,
    {
        self.left = Some(Box::new(node.into()));
        self
    }

    pub fn right<N>(mut self, node: N) -> Self
    where
        N: Into<Node<T>>,
        T: std::fmt::Debug,
    {
        self.right = Some(Box::new(node.into()));
        self
    }
}

pub fn root(value: i32) -> Node<i32> {
    Node::new(value)
}

impl<T> Node<T> {
    pub fn create_unbalanced_tree() -> NodeRef<i32> {
        let root = root(1).left(root(2).left(4).right(5)).right(3);
        Box::new(root)
    }

    pub fn insert(&mut self, value: T) {
        let mut queue: VecDeque<&mut Node<T>> = VecDeque::new();
        queue.push_front(self);
        loop {
            let Node {
                ref mut left,
                ref mut right,
                ..
            } = queue.pop_back().unwrap();
            match left {
                Some(node) => queue.push_front(node),
                None => {
                    *left = Some(Box::new(Node::<T>::new(value)));
                    return;
                }
            }
            match right {
                Some(node) => queue.push_front(node),
                None => {
                    *right = Some(Box::new(Node::<T>::new(value)));
                    return;
                }
            }
        }
    }
}

pub mod stack_traversal {
    use super::NodeRef;

    //pub fn post_order(root: &NodeRef<i32>, buffer: &mut Vec<i32>) {}
    pub fn in_order(root: &NodeRef<i32>, buffer: &mut Vec<i32>) {
        let mut stack = Vec::<&NodeRef<i32>>::new();
        let mut curr = Some(root);

        while curr.is_some() || !stack.is_empty() {
            while curr.is_some() {
                stack.push(curr.unwrap());
                curr = curr.unwrap().left.as_ref();
            }

            curr = stack.pop();
            buffer.push(curr.unwrap().value);
            curr = curr.unwrap().right.as_ref();
        }
    }
}

pub mod traversal {
    use super::OptionNodeRef;

    pub fn post_order(root: OptionNodeRef<i32>, buffer: &mut Vec<i32>) {
        // TODO(pplanel): how to get the len of tree by level
        match root {
            Some(node) => {
                post_order(node.left, buffer);
                post_order(node.right, buffer);
                buffer.push(node.value);
            }
            None => {}
        }
    }

    pub fn pre_order(root: OptionNodeRef<i32>, buffer: &mut Vec<i32>) {
        // TODO(pplanel): how to get the len of tree by level
        match root {
            Some(node) => {
                buffer.push(node.value);
                pre_order(node.left, buffer);
                pre_order(node.right, buffer);
            }
            None => {}
        }
    }

    pub fn in_order(root: OptionNodeRef<i32>, buffer: &mut Vec<i32>) {
        // TODO(pplanel): how to get the len of tree by level
        match root {
            Some(node) => {
                in_order(node.left, buffer);
                buffer.push(node.value);
                in_order(node.right, buffer);
            }
            None => {}
        }
    }
}
