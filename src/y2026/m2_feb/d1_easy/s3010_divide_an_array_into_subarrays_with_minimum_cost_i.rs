// LeetCode Problem 3010. Divide an Array Into Subarrays With Minimum Cost I
// Difficulty: Easy
//
// Time Complexity: O(n log n) - n is the size of the array, sorting time is logarithmic
// Space Complexity: O(1) - space utilised is always constant

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
        let first = nums.remove(0);
        nums.sort();
        first + nums[0] + nums[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost_1() {
        let input = vec![1, 2, 3, 12];
        let result = Solution::minimum_cost(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_minimum_cost_2() {
        let input = vec![5, 4, 3];
        let result = Solution::minimum_cost(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_minimum_cost_3() {
        let input = vec![10, 3, 1, 1];
        let result = Solution::minimum_cost(input);
        assert_eq!(result, 12);
    }
}
