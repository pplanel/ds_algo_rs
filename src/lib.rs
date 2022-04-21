pub mod binary_tree;

#[cfg(test)]
mod binary_tree_traversal {
    use crate::binary_tree::{
        traversal::{in_order, post_order, pre_order},
        Node,
    };

    #[test]
    fn test_in_order() {
        let level = 2;
        let mut buffer = Vec::with_capacity(level + 1);
        let mut counter = 1;
        let root = Node::<i32>::generate_tree(level, &mut counter);
        in_order(root, level, &mut buffer);
        assert_eq!(buffer, vec![2, 1, 3]);
    }

    #[test]
    fn test_pre_order() {
        let level = 2;
        let mut buffer = Vec::with_capacity(level + 1);
        let mut counter = 1;
        let root = Node::<i32>::generate_tree(level, &mut counter);
        pre_order(root, level, &mut buffer);
        assert_eq!(buffer, vec![1, 2, 3]);
    }

    #[test]
    fn test_post_order() {
        let mut buffer = Vec::with_capacity(5);
        let root = Node::<i32>::create_unbalanced_tree();
        post_order(root, &mut buffer);
        assert_eq!(buffer, vec![4, 5, 2, 3, 1]);
    }
}
