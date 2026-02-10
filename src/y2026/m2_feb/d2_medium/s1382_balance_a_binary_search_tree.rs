// LeetCode Problem 1382. Balance a Binary Search Tree
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of the binary tree
// Space Complexity: O(n) - where n is the number of nodes in a binary tree

#![allow(dead_code)]

use crate::utils::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

struct Solution;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut st = vec![];
        Self::dfs(root, &mut st);
        Self::build(&st)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, st: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root {
            let mut number = root.borrow_mut();
            let (left, right) = (number.left.take(), number.right.take());
            drop(number);

            Self::dfs(left, st);
            st.push(root);
            Self::dfs(right, st);
        }
    }

    fn build(arr: &[Rc<RefCell<TreeNode>>]) -> Option<Rc<RefCell<TreeNode>>> {
        match arr {
            [] => None,
            [x] => Some(Rc::clone(x)),
            _ => {
                let mid = arr.len() / 2;
                let mut root = arr[mid].borrow_mut();
                root.left = Self::build(&arr[..mid]);
                root.right = Self::build(&arr[mid + 1..]);

                Some(arr[mid].clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::build_tree::build_tree;

    use super::*;

    #[test]
    fn test_balance_bst_1() {
        let input = build_tree(vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
        ]);
        let result = Solution::balance_bst(input);
        let expected = build_tree(vec![Some(3), Some(2), Some(4), Some(1)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_balance_bst_2() {
        let input = build_tree(vec![Some(2), Some(1), Some(3)]);
        let result = Solution::balance_bst(input);
        let expected = build_tree(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(result, expected);
    }
}
