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
        //TODO
        match self.root {
            None => {
                // 如果树为空，创建根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(ref mut root) => {
                // 否则，在树中插入节点
                root.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            None => false,
            Some(node) => {
                // 比较当前节点值与目标值
                match value.cmp(&node.value) {
                    Ordering::Equal => true,
                    Ordering::Less => {
                        // 如果目标值小于当前节点值，在左子树中搜索
                        match &node.left {
                            None => false,
                            Some(left) => {
                                // 直接递归搜索左子树，不创建新的树实例
                                Self::search_in_node(left, &value)
                            }
                        }
                    }
                    Ordering::Greater => {
                        // 如果目标值大于当前节点值，在右子树中搜索
                        match &node.right {
                            None => false,
                            Some(right) => {
                                // 直接递归搜索右子树，不创建新的树实例
                                Self::search_in_node(right, &value)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    // 辅助方法，在节点中搜索值
    fn search_in_node(node: &Box<TreeNode<T>>, value: &T) -> bool {
        match value.cmp(&node.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                match &node.left {
                    None => false,
                    Some(left) => Self::search_in_node(left, value)
                }
            }
            Ordering::Greater => {
                match &node.right {
                    None => false,
                    Some(right) => Self::search_in_node(right, value)
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
        match value.cmp(&self.value) {
            Ordering::Equal => {
                // 值已存在，不做任何操作
                return;
            }
            Ordering::Less => {
                // 如果值小于当前节点值，插入到左子树
                match self.left {
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(ref mut left) => {
                        left.insert(value);
                    }
                }
            }
            Ordering::Greater => {
                // 如果值大于当前节点值，插入到右子树
                match self.right {
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(ref mut right) => {
                        right.insert(value);
                    }
                }
            }
        }
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


