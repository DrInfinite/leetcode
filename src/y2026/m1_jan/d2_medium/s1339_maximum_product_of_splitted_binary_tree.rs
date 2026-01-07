// LeetCode Problem 1339. Maximum Product of Splitted Binary Tree
// Difficulty: Medium

// Time Complexity: O(n) - where n is the number of nodes in the binary tree.
// The tree is traversed twice: once to compute the total sum of all nodes
// and once using DFS to compute subtree sums and track the maximum product.
// Each node is processed a constant number of times, resulting in O(n) time.

// Space Complexity: O(n) - where n is the number of nodes in the tree.
// This includes O(n) space for the queue used in the first traversal and
// O(h) space for the recursion stack in the DFS helper function (h = height
// of the tree, worst case n for a skewed tree).

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::lib::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut queue = vec![root.clone()];

        while !queue.is_empty() {
            if let Some(Some(v)) = queue.pop() {
                let v = v.borrow();
                sum += v.val as i64;
                queue.push(v.left.clone());
                queue.push(v.right.clone());
            }
        }

        fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: i64) -> (i64, i64) {
            let Some(root) = root else {
                return (0, 0);
            };
            let root = root.borrow();
            let (ls, lm) = helper(root.left.clone(), sum);
            let (rs, rm) = helper(root.right.clone(), sum);

            let own_sum = root.val as i64 + ls + rs;
            let can = own_sum * (sum - own_sum);

            (own_sum, can.max(lm.max(rm)))
        }

        (helper(root, sum).1 % 1000000007) as i32
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
    fn test_example_1() {
        // Input: root = [1,2,3,4,5,6]
        //
        //       1
        //      / \
        //     2   3
        //    / \  /
        //   4  5 6

        let n1 = node(1);
        let n2 = node(2);
        let n3 = node(3);
        let n4 = node(4);
        let n5 = node(5);
        let n6 = node(6);

        n1.borrow_mut().left = Some(n2.clone());
        n1.borrow_mut().right = Some(n3.clone());
        n2.borrow_mut().left = Some(n4);
        n2.borrow_mut().right = Some(n5);
        n3.borrow_mut().left = Some(n6);

        let root = Some(n1);

        assert_eq!(Solution::max_product(root), 110);
    }

    #[test]
    fn test_example_2() {
        // Input: root = [1,null,2,3,4,null,null,5,6]
        //
        //       1
        //        \
        //         2
        //        / \
        //       3   4
        //      / \
        //     5   6

        let n1 = node(1);
        let n2 = node(2);
        let n3 = node(3);
        let n4 = node(4);
        let n5 = node(5);
        let n6 = node(6);

        n1.borrow_mut().right = Some(n2.clone());
        n2.borrow_mut().left = Some(n3.clone());
        n2.borrow_mut().right = Some(n4);
        n3.borrow_mut().left = Some(n5);
        n3.borrow_mut().right = Some(n6);

        let root = Some(n1);

        assert_eq!(Solution::max_product(root), 90);
    }
}
