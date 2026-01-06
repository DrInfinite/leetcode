// LeetCode Problem 1161. Maximum Level Sum of a Binary Tree
// Difficulty: Medium

// Time Complexity: O(n) - where n is the number of nodes in the binary tree.
// Each node is visited exactly once during the DFS traversal, and the final
// loop over the level sums also runs in O(n) in the worst case (skewed tree).

// Space Complexity: O(n) - where n is the number of nodes in the tree.
// This includes O(h) space for the recursion stack (h = height of the tree,
// worst case n) and O(n) space for the `sums` vector storing level-wise sums.

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::lib::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;

        let mut sums: Vec<i32> = Vec::new();
        Self::dfs(&root, 0, &mut sums);

        let mut max_sum = i32::MIN;

        for (i, &sum) in sums.iter().enumerate() {
            if max_sum < sum {
                max_sum = sum;
                res = i + 1;
            }
        }

        res as i32
    }

    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, lv: usize, sums: &mut Vec<i32>) {
        if let Some(n) = node {
            let n_borrow = n.as_ref().borrow();
            if sums.len() == lv {
                sums.push(0);
            }
            sums[lv] += n_borrow.val;
            Self::dfs(&n_borrow.left, lv + 1, sums);
            Self::dfs(&n_borrow.right, lv + 1, sums);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    #[test]
    fn test_max_level_sum_1() {
        // Input: [1,7,0,7,-8,null,null]
        // Tree:
        //        1
        //       / \
        //      7   0
        //     / \
        //    7  -8

        let root = node(1);
        let left = node(7);
        let right = node(0);
        let left_left = node(7);
        let left_right = node(-8);

        root.borrow_mut().left = Some(left.clone());
        root.borrow_mut().right = Some(right);

        left.borrow_mut().left = Some(left_left);
        left.borrow_mut().right = Some(left_right);

        let result = Solution::max_level_sum(Some(root));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_max_level_sum_2() {
        // Input: [989,null,10250,98693,-89388,null,null,null,-32127]
        // Tree:
        //        989
        //          \
        //          10250
        //          /   \
        //      98693  -89388
        //         \
        //        -32127

        let root = node(989);
        let right = node(10250);
        let right_left = node(98693);
        let right_right = node(-89388);
        let deep_right = node(-32127);

        root.borrow_mut().right = Some(right.clone());
        right.borrow_mut().left = Some(right_left.clone());
        right.borrow_mut().right = Some(right_right);
        right_left.borrow_mut().right = Some(deep_right);

        let result = Solution::max_level_sum(Some(root));
        assert_eq!(result, 2);
    }
}
