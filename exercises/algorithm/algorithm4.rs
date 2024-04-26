/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/
use std::cmp::Ordering;
use std::fmt::Debug;


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
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let _node = Box::new(TreeNode::new(value)); 
        match &self.root{
            Some(_temp) =>{
                let mut _index = &mut self.root;                
                loop {
                    match _index{
                        Some(x)=>{
                                if x.value  == _node.value{
                                    return;
                                }
                            if x.value < _node.value{                                
                                match &x.right{
                                    Some(p) =>{},
                                    Node =>{
                                        x.right = Some(_node);
                                        return;
                                    }
                                }
                                _index = &mut x.right;

                            }else{
                                match &x.left{
                                    Some(p) =>{},
                                    Node =>{
                                        x.left = Some(_node);
                                        return;
                                    }
                                }
                                _index = &mut x.left;                               
                            }
                        },
                        None =>{
                            break;
                        }
                    }
                }
                
            },
            None =>{
                self.root = Some(_node);
            }
        }        
        //TODO

    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut _index = & self.root;
        loop{
            match _index{
                None =>{
                    return false;
                },
                Some(_p)=>{
                    if _p.value == value{
                        return true;
                    }else if _p.value < value{
                        _index = &_p.right;
                    }else{
                        _index = &_p.left;
                    }
                }
            }
        }

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


