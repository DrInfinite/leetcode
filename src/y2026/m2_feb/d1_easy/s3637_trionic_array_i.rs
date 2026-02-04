// LeetCode Problem 3637. Trionic Array I
// Difficulty: Easy
//
// Time Complexity: O(n) - The algorithm makes a single pass through the array,
// comparing each element with its predecessor once.
//
// Space Complexity: O(1) - Only a constant amount of extra space is used for
// variables, regardless of input size.

#![allow(dead_code)]

struct Solution;
use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut changes = 0;
        if nums.len() < 3 {
            return false;
        }

        if nums[1] <= nums[0] {
            return false;
        }

        for i in 1..nums.len() {
            if changes >= 3 {
                return false;
            }

            match nums[i].cmp(&nums[i - 1]) {
                Greater => {
                    if (changes & 1) == 1 {
                        changes += 1;
                    }
                }
                Equal => return false,
                Less => {
                    if (changes & 1) == 0 {
                        changes += 1;
                    }
                }
            }
        }

        changes == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_trionic_1() {
        let input = vec![1, 3, 5, 4, 2, 6];
        let result = Solution::is_trionic(input);
        assert!(result);
    }

    #[test]
    fn test_is_trionic_2() {
        let input = vec![2, 1, 3];
        let result = Solution::is_trionic(input);
        assert!(!result);
    }
}
