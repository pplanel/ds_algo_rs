pub mod binary_tree;

#[cfg(test)]
mod binary_tree_traversal {
    use crate::binary_tree::{root, stack_traversal, traversal, Node};

    #[test]
    fn test_insert_fb() {
        let mut tree = Node::new(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);
        assert_eq!(tree, root(1).left(root(2).left(4).right(5)).right(3));
        tree.insert(6);
        assert_eq!(
            tree,
            root(1)
                .left(root(2).left(4).right(5))
                .right(root(3).left(6))
        );
    }

    #[test]
    fn test_in_order_stack() {
        let mut buffer = Vec::new();
        let root = Node::<i32>::create_unbalanced_tree();
        stack_traversal::in_order(&root, &mut buffer);
        assert_eq!(buffer, vec![4, 2, 5, 1, 3]);
    }

    //#[test]
    //fn test_post_order_stack() {
    //    let mut buffer = Vec::new();
    //    let root = Node::<i32>::create_unbalanced_tree();
    //    stack_traversal::post_order(&root, &mut buffer);
    //    assert_eq!(buffer, vec![4, 5, 2, 3, 1]);
    //}

    #[test]
    fn test_in_order() {
        let mut buffer = Vec::new();
        let root = Node::<i32>::create_unbalanced_tree();
        traversal::in_order(Some(root), &mut buffer);
        assert_eq!(buffer, vec![4, 2, 5, 1, 3]);
    }

    #[test]
    fn test_pre_order() {
        let mut buffer = Vec::new();
        let root = Node::<i32>::create_unbalanced_tree();
        traversal::pre_order(Some(root), &mut buffer);
        assert_eq!(buffer, vec![1, 2, 4, 5, 3]);
    }

    #[test]
    fn test_post_order() {
        let mut buffer = Vec::with_capacity(5);
        let root = Node::<i32>::create_unbalanced_tree();
        traversal::post_order(Some(root), &mut buffer);
        assert_eq!(buffer, vec![4, 5, 2, 3, 1]);
    }
}
