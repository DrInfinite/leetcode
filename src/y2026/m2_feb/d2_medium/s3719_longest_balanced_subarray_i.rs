// LeetCode Problem 3719. Longest Balanced Subarray I
// Difficulty: Medium
//
// Time Complexity: O(n^2) - where n is the size of the input array nums
// Space Complexity: O(n) - where n is the number of distinct elements in the array nums

#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        for i in 0..n {
            let mut even_freq: HashMap<i32, i32> = HashMap::new();
            let mut odd_freq: HashMap<i32, i32> = HashMap::new();

            let mut distinct_even = 0;
            let mut distinct_odd = 0;

            for j in i..n {
                let x = nums[j];

                if x % 2 == 0 {
                    let entry = even_freq.entry(x).or_insert(0);

                    if *entry == 0 {
                        distinct_even += 1;
                    }

                    *entry += 1;
                } else {
                    let entry = odd_freq.entry(x).or_insert(0);

                    if *entry == 0 {
                        distinct_odd += 1;
                    }

                    *entry += 1;
                }

                if distinct_even == distinct_odd {
                    ans = ans.max((j - i + 1) as i32);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_balanced_1() {
        let input = vec![2, 5, 4, 3];
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 4)
    }
    #[test]
    fn test_longest_balanced_2() {
        let input = vec![3, 2, 2, 5, 4];
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 5)
    }
    #[test]
    fn test_longest_balanced_3() {
        let input = vec![1, 2, 3, 2];
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 3)
    }
}
