// LeetCode Problem 1458. Max Dot Product of Two Subsequences
// Difficulty: Hard

// Time Complexity: O(n^3), where n = max(nums1.len(), nums2.len())

// Space Complexity: O(n^2), due to the DP table of size n × n

#![allow(dead_code)]

use std::cmp::max;
struct Solution;

impl Solution {
    pub fn recursive(
        nums1: &Vec<i32>,
        nums2: &Vec<i32>,
        dp: &mut Vec<Vec<i32>>,
        index_left: usize,
        index_right: usize,
    ) -> i32 {
        if index_left >= nums1.len() || index_right >= nums2.len() {
            return i32::MIN;
        }

        if dp[index_left][index_right] != i32::MIN {
            return dp[index_left][index_right];
        }

        let mut right_val = i32::MIN;

        for i in index_right..nums2.len() {
            let out1 = Self::recursive(nums1, nums2, dp, index_left + 1, i + 1);
            let out2 = Self::recursive(nums1, nums2, dp, index_left + 1, i);
            let curr = nums1[index_left] * nums2[i];

            match out1 == i32::MIN {
                true => {
                    right_val = max(right_val, curr);
                }
                false => {
                    right_val = max(out1, max(curr + out1, max(right_val, curr)));
                }
            }

            if out2 != i32::MIN {
                right_val = max(out2, right_val);
            }
        }

        dp[index_left][index_right] = right_val;

        right_val
    }

    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let max_len = max(nums1.len(), nums2.len());
        let mut dp = vec![vec![i32::MIN; max_len]; max_len];
        Self::recursive(&nums1, &nums2, &mut dp, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_dot_product_1() {
        let nums1 = vec![2, 1, -2, 5];
        let nums2 = vec![3, 0, -6];
        let result = Solution::max_dot_product(nums1, nums2);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_max_dot_product_2() {
        let nums1 = vec![3, -2];
        let nums2 = vec![2, -6, 7];
        let result = Solution::max_dot_product(nums1, nums2);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_max_dot_product_3() {
        let nums1 = vec![-1, -1];
        let nums2 = vec![1, 1];
        let result = Solution::max_dot_product(nums1, nums2);
        assert_eq!(result, -1);
    }
}
