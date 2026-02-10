// LeetCode Problem 110. Balanced Binary Tree
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the size of the binary tree
// Space Complexity: O(n) - where n is the number of nodes in a binary tree

#![allow(dead_code)]

use std::{cell::RefCell, cmp, rc::Rc};

use crate::utils::tree_node::TreeNode;
struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root) != -1
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                let (left, right) = (Self::dfs(&n.left), Self::dfs(&n.right));

                if left == -1 || right == -1 || (left - right).abs() > 1 {
                    return -1;
                }

                1 + cmp::max(left, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::build_tree::build_tree;

    use super::*;

    #[test]
    fn test_is_balanced_1() {
        let input = build_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::is_balanced(input);

        assert!(result);
    }

    #[test]
    fn test_is_balanced_2() {
        let input = build_tree(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        let result = Solution::is_balanced(input);

        assert!(!result);
    }

    #[test]
    fn test_is_balanced_3() {
        let input = build_tree(vec![]);
        let result = Solution::is_balanced(input);

        assert!(result);
    }
}
