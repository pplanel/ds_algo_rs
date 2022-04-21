pub type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Box<Node<T>> {
        Box::new(Node {
            value,
            left: None,
            right: None,
        })
    }

    pub fn generate_tree(level: usize, counter: &mut i32) -> NodeRef<i32> {
        if level == 0 {
            None
        } else {
            let mut root = Node {
                value: *counter,
                left: None,
                right: None,
            };
            *counter += 1;
            root.left = Self::generate_tree(level - 1, counter);
            root.right = Self::generate_tree(level - 1, counter);
            Some(Box::new(root))
        }
    }

    pub fn create_unbalanced_tree() -> NodeRef<i32> {
        let mut root = Node::new(1);
        let mut left_node = Some(Node::new(2));
        match left_node {
            Some(ref mut node) => {
                node.left = Some(Node::new(4));
                node.right = Some(Node::new(5));
            }
            None => {}
        }
        root.left = left_node;
        root.right = Some(Node::new(3));
        Some(root)
    }
}

pub mod traversal {
    use super::NodeRef;

    pub fn post_order(root: NodeRef<i32>, buffer: &mut Vec<i32>) {
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

    pub fn pre_order(root: NodeRef<i32>, level: usize, buffer: &mut Vec<i32>) {
        // TODO(pplanel): how to get the len of tree by level
        match root {
            Some(node) => {
                buffer.push(node.value);
                pre_order(node.left, level + 1, buffer);
                pre_order(node.right, level + 1, buffer);
            }
            None => {}
        }
    }

    pub fn in_order(root: NodeRef<i32>, level: usize, buffer: &mut Vec<i32>) {
        // TODO(pplanel): how to get the len of tree by level
        match root {
            Some(node) => {
                in_order(node.left, level + 1, buffer);
                buffer.push(node.value);
                in_order(node.right, level + 1, buffer);
            }
            None => {}
        }
    }
}
