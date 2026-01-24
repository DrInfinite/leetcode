// LeetCode Problem 865. Smallest Subtree with all the Deepest Nodes
// Difficulty: Medium

// Time Complexity: O(n) - Each node in the binary tree is visited exactly once
// during the DFS traversal.

// Space Complexity: O(h) - Space is used only by the recursion call stack,
// where h is the height of the tree.

#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use crate::utils::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).1
    }

    pub fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(n) => {
                let n_ref = n.borrow();
                let (r_depth, r_ref) = Self::dfs(&n_ref.left);
                let (l_depth, l_ref) = Self::dfs(&n_ref.right);

                if r_depth > l_depth {
                    return (r_depth + 1, r_ref);
                } else if r_depth < l_depth {
                    return (l_depth + 1, l_ref);
                }

                (l_depth + 1, node.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::build_tree::build_tree;

    use super::*;

    #[test]
    fn test_subtree_with_all_deepest_1() {
        // Input: [3,5,1,6,2,0,8,null,null,7,4]
        let root = build_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);

        let result = Solution::subtree_with_all_deepest(root);

        // Expected output: subtree rooted at 2 (with children 7 and 4)
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_subtree_with_all_deepest_2() {
        // Input: [1]
        let root = build_tree(vec![Some(1)]);

        let result = Solution::subtree_with_all_deepest(root);

        // Expected output: [1]
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_subtree_with_all_deepest_3() {
        // Input: [0,1,3,null,2]
        let root = build_tree(vec![Some(0), Some(1), Some(3), None, Some(2)]);

        let result = Solution::subtree_with_all_deepest(root);

        // Expected output: [2]
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
