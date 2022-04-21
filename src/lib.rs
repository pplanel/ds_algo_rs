pub mod binary_tree;

#[cfg(test)]
mod binary_tree_traversal {
    use crate::binary_tree::{
        traversal::{in_order, post_order, pre_order},
        Node,
    };

    #[test]
    fn test_in_order() {
        let mut buffer = Vec::new();
        let root = Node::<i32>::generate_tree(3);
        in_order(root, &mut buffer);
        assert_eq!(buffer, vec![4, 2, 5, 1, 3]);
    }

    #[test]
    fn test_pre_order() {
        let mut buffer = Vec::new();
        let root = Node::<i32>::generate_tree(3);
        pre_order(root, &mut buffer);
        assert_eq!(buffer, vec![1, 2, 4, 5, 3]);
    }

    #[test]
    fn test_post_order() {
        let mut buffer = Vec::with_capacity(5);
        let root = Node::<i32>::create_unbalanced_tree();
        post_order(Some(root), &mut buffer);
        assert_eq!(buffer, vec![4, 5, 2, 3, 1]);
    }
}
