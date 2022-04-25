use std::cmp;

#[derive(Debug, Eq, PartialOrd, Ord)]
pub enum BinarySearchTree<T: Ord> {
    Leaf {
        value: T,
        left: Box<BinarySearchTree<T>>,
        right: Box<BinarySearchTree<T>>,
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
                Self::Leaf {
                    value: l_value,
                    left: l_left,
                    right: l_right,
                },
                Self::Leaf {
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

    pub fn height(self) -> usize {
        if let BinarySearchTree::Leaf {
            value: _,
            left,
            right,
        } = self
        {
            let count_left = Self::height(*left);
            let count_right = Self::height(*right);
            let height = cmp::max(count_left, count_right);
            height + 1
        } else {
            0
        }
    }

    pub fn create(value: T) -> Self {
        BinarySearchTree::Leaf {
            value,
            left: Box::new(BinarySearchTree::Empty),
            right: Box::new(BinarySearchTree::Empty),
        }
    }

    pub fn insert(&mut self, new_value: T) {
        match self {
            BinarySearchTree::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.cmp(value) {
                std::cmp::Ordering::Less => left.insert(new_value),
                std::cmp::Ordering::Greater => right.insert(new_value),
                std::cmp::Ordering::Equal => (),
            },
            BinarySearchTree::Empty => *self = BinarySearchTree::create(new_value),
        }
    }

    pub fn sorted_values(&self) -> Vec<&T> {
        let mut values = Vec::new();

        match self {
            BinarySearchTree::Leaf {
                ref value,
                ref left,
                ref right,
            } => {
                values.extend(left.sorted_values());
                values.push(value);
                values.extend(right.sorted_values());
            }
            BinarySearchTree::Empty => {}
        }

        values
    }
}

#[allow(dead_code)]
fn print_tree<T>(root: &BinarySearchTree<T>)
where
    T: Ord + std::fmt::Display,
{
    if let BinarySearchTree::Leaf { value, left, right } = root {
        print_tree(left);
        println!("{}", value);
        print_tree(right);
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
        let root = BinarySearchTree::create(1_usize);
        match root {
            BinarySearchTree::Leaf { value, left, right } => {
                assert_eq!(value, 1_usize);
                assert_eq!(*left, BinarySearchTree::Empty);
                assert_eq!(*right, BinarySearchTree::Empty);
            }
            BinarySearchTree::Empty => panic!("BinarySearchTree::create returned Empty"),
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
        let mut root = BinarySearchTree::create(5);
        root.insert(3);
        root.insert(6);
        root.insert(2);
        root.insert(4);
        root.insert(1);
        assert_eq!(root.height(), 4);
    }

    #[test]
    fn test_print_bst() {
        let mut root = BinarySearchTree::create(5);
        root.insert(3);
        root.insert(6);
        root.insert(2);
        root.insert(4);
        root.insert(1);
        print_tree(&root)
    }
}
