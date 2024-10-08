/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone + Display
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        println!("Doing insert!");
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value.clone())));
            return;
        }

        if self.search(value.clone()) {
            return;
        }


        let mut new_node = TreeNode::new(value.clone());
        let mut node = self.root.as_mut().unwrap();
        while node.left.is_some() || node.right.is_some() {
            if node.value > new_node.value {
                if node.left.is_none() {
                    break;
                }
                node = node.left.as_mut().unwrap();
            }
            else {
                if node.right.is_none() {
                    break;
                }
                node = node.right.as_mut().unwrap();
            }
        }
        if node.value > new_node.value {
            node.left = Some(Box::new(new_node));
        }
        else if node.value == new_node.value {
            println!("Duplicate value: {}", value.clone());
        }
        else {
            node.right = Some(Box::new(new_node));
        }
        println!("Inserted: {}", value.clone());
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if self.root.is_none() {
            return false;
        }
        let mut node = self.root.as_ref().unwrap();
        while node.value != value {
            if node.value > value {
                if node.left.is_none() {
                    return false;
                }
                node = node.left.as_ref().unwrap();
            }
            else {
                if node.right.is_none() {
                    return false;
                }
                node = node.right.as_ref().unwrap();
            }
        }
        true
}
}
impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


