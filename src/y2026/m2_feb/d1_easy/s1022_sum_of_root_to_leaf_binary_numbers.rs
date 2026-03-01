// LeetCode Problem 1022. Sum of Root To Leaf Binary Numbers
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the size of the binary tree
// Space Complexity: O(n) - where n is the worst-case size of the binary tree

#![allow(dead_code)]
use crate::utils::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

struct Solution;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |n| {
            let n = n.borrow_mut();
            [&n.left, &n.right]
                .into_iter()
                .flatten()
                .map(|x| {
                    x.borrow_mut().val += n.val * 2;
                    Self::sum_root_to_leaf(Some(x.clone()))
                })
                .sum::<i32>()
                .max(n.val)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::utils::build_tree::build_tree;

    use super::*;

    #[test]
    fn test_sum_root_to_leaf_1() {
        let input = build_tree(vec![
            Some(1),
            Some(0),
            Some(1),
            Some(0),
            Some(1),
            Some(0),
            Some(1),
        ]);
        let result = Solution::sum_root_to_leaf(input);
        assert_eq!(result, 22)
    }

    #[test]
    fn test_sum_root_to_leaf_2() {
        let input = build_tree(vec![Some(0)]);
        let result = Solution::sum_root_to_leaf(input);
        assert_eq!(result, 0)
    }
}
