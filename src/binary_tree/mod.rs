use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Eq, PartialOrd, Ord)]
pub enum BinarySearchTree<T: Ord> {
    Node {
        value: T,
        left: Option<Box<BinarySearchTree<T>>>,
        right: Option<Box<BinarySearchTree<T>>>,
    },
    Empty,
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> PartialEq for BinarySearchTree<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Node {
                    value: l_value,
                    left: l_left,
                    right: l_right,
                },
                Self::Node {
                    value: r_value,
                    left: r_left,
                    right: r_right,
                },
            ) => l_value == r_value && l_left == r_left && l_right == r_right,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree::Empty
    }

    pub fn height(&self) -> usize {
        match self {
            BinarySearchTree::Node { left, right, .. } => {
                let hl = match left {
                    Some(ref node) => node.height(),
                    None => 0,
                };
                let hr = match right {
                    Some(ref node) => node.height(),
                    None => 0,
                };
                std::cmp::max(hl, hr) + 1
            }
            BinarySearchTree::Empty => 0,
        }
    }

    pub fn create(value: T) -> Self {
        BinarySearchTree::Node {
            value,
            left: Some(Box::new(BinarySearchTree::Empty)),
            right: Some(Box::new(BinarySearchTree::Empty)),
        }
    }

    pub fn insert(&mut self, new_value: T) {
        match self {
            BinarySearchTree::Node {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.cmp(value) {
                std::cmp::Ordering::Less => match left {
                    Some(node) => node.insert(new_value),
                    None => (),
                },
                std::cmp::Ordering::Greater => match right {
                    Some(node) => node.insert(new_value),
                    None => (),
                },
                std::cmp::Ordering::Equal => (),
            },
            BinarySearchTree::Empty => *self = BinarySearchTree::create(new_value),
        }
    }

    pub fn sorted_values(&self) -> Vec<&T> {
        let mut values = Vec::new();

        match self {
            BinarySearchTree::Node {
                ref value,
                ref left,
                ref right,
            } => match (left, right) {
                (None, None) => todo!(),
                (None, Some(right)) => values.extend(right.sorted_values()),
                (Some(left), None) => values.extend(left.sorted_values()),
                (Some(left), Some(right)) => {
                    values.extend(left.sorted_values());
                    values.push(value);
                    values.extend(right.sorted_values());
                }
            },
            BinarySearchTree::Empty => {}
        }

        values
    }
}

//impl<T: Ord> std::fmt::Display for BinarySearchTree<T> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        let height = self.height();
//        let max_nodes = usize::pow(2usize, height as u32 + 1);
//    }
//}

#[allow(dead_code)]
fn print_tree<T>(root: &BinarySearchTree<T>)
where
    T: Ord + std::fmt::Display,
{
    let mut queue = VecDeque::new();
    let mut stack: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let level = 1usize;
    queue.push_back((root, level));
    while !queue.is_empty() {
        let (curr, level) = queue.pop_front().unwrap();
        if let BinarySearchTree::Node {
            value,
            left: Some(left),
            right: Some(right),
        } = curr
        {
            stack
                .entry(level)
                .or_insert_with(Vec::new)
                .push(value.to_string());
            queue.push_back((left.as_ref(), level + 1));
            queue.push_back((right.as_ref(), level + 1));
        }
    }
    drop(queue);
    let max_nodes = |level| usize::pow(2usize, level as u32 - 1);
    let padding = 1;
    let width = max_nodes(root.height()) * padding;
    dbg!(max_nodes(root.height()), width, root.height());
    for (level, values) in stack.iter() {
        let line = values
            .iter()
            .map(|v| (v, width * v.len()))
            .map(|(v, width)| format!("{:^w$}", v, w = width / max_nodes(*level)))
            .reduce(|cur: String, nxt: String| cur + &nxt)
            .unwrap();
        println!("{}", line)
    }
}

#[cfg(test)]
mod test_bst {

    use crate::binary_tree::BinarySearchTree;

    use super::print_tree;

    #[test]
    fn test_new_bst() {
        assert_eq!(BinarySearchTree::<i32>::new(), BinarySearchTree::Empty);
    }

    #[test]
    fn test_create_bst() {
        if let BinarySearchTree::Node {
            value,
            left: Some(left),
            right: Some(right),
        } = BinarySearchTree::create(1)
        {
            assert_eq!(value, 1);
            assert_eq!(*left, BinarySearchTree::Empty);
            assert_eq!(*right, BinarySearchTree::Empty);
        } else {
            panic!("BinarySearchTree::create returned Empty")
        }
    }

    #[test]
    fn test_insert_bst() {
        let mut root = BinarySearchTree::create(1_usize);
        root.insert(2_usize);
        root.insert(3_usize);
        root.insert(4_usize);
        root.insert(5_usize);
        assert_eq!(
            root.sorted_values(),
            vec![&1_usize, &2_usize, &3_usize, &4_usize, &5_usize]
        );
    }

    #[test]
    fn test_height_bst() {
        let mut root = BinarySearchTree::create(30);
        root.insert(18);
        root.insert(50);
        root.insert(24);
        root.insert(36);
        root.insert(51);
        root.insert(17);
        root.insert(16);
        root.insert(15);
        assert_eq!(root.height(), 4);
    }

    #[test]
    fn test_match() {
        pub enum Choice {
            One { a: Option<bool>, b: Option<bool> },
        }
        let a = Choice::One {
            a: None,
            b: Some(true),
        };
        if let Choice::One {
            a: Some(a),
            b: Some(b),
        } = a
        {
            println!("{} {}", a, b);
        };
    }

    #[test]
    fn test_print_bst() {
        let mut root = BinarySearchTree::create(30);
        root.insert(18);
        root.insert(50);
        root.insert(24);
        root.insert(36);
        root.insert(51);
        root.insert(17);
        root.insert(16);
        root.insert(15);
        root.insert(19);
        print_tree(&root)
    }
}
