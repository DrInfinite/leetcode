// LeetCode Problem 1877. Minimize Maximum Pair Sum in Array
// Difficulty: Medium
//
// Time Complexity:
// Space Complexity:

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() >> 1..]
            .iter()
            .zip(nums[..nums.len() >> 1].iter().rev())
            .fold(0, |m, (l, r)| m.max(l + r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_pair_sum_1() {
        let input = vec![3, 5, 2, 3];
        let result = Solution::min_pair_sum(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_min_pair_sum_2() {
        let input = vec![3, 5, 4, 2, 4, 6];
        let result = Solution::min_pair_sum(input);
        assert_eq!(result, 8);
    }
}
