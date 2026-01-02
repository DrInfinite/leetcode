// LeetCode Problem 961. N-Repeated Element in Size 2N Array
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the array
// Space Complexity: O(n) - where n is the number of elements in the array

#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();

        for num in nums {
            if !seen.insert(num) {
                return num;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_n_times_1() {
        let nums = vec![1, 2, 3, 3];
        assert_eq!(Solution::repeated_n_times(nums), 3);
    }

    #[test]
    fn test_repeated_n_times_2() {
        let nums = vec![2, 1, 2, 5, 3, 2];
        assert_eq!(Solution::repeated_n_times(nums), 2);
    }

    #[test]
    fn test_repeated_n_times_3() {
        let nums = vec![5, 1, 5, 2, 5, 3, 5, 4];
        assert_eq!(Solution::repeated_n_times(nums), 5);
    }
}
