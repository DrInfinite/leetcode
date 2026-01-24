// LeetCode Problem 3507. Minimum Pair Removal to Sort Array I
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the vector nums
// Space Complexity: O(n) - where n is the length of the vector nums

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;

        while !nums.is_sorted() {
            let i = (1..nums.len())
                .min_by_key(|&k| nums[k] + nums[k - 1])
                .unwrap();
            nums[i - 1] += nums.remove(i);
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_pair_removal_1() {
        let input = vec![5, 2, 3, 1];
        let result = Solution::minimum_pair_removal(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_minimum_pair_removal_2() {
        let input = vec![1, 2, 2];
        let result = Solution::minimum_pair_removal(input);
        assert_eq!(result, 0);
    }
}
