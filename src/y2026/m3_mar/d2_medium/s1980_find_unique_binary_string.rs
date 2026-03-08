// LeetCode Problem 1980. Find Unique Binary String
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of the string
// Space Complexity: O(1) - constant space, no new data structures generated

#![allow(dead_code)]
struct Solution;

impl Solution {
    // always evals to the smallest possible binary number
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        (0..nums.len())
            .map(|i| char::from(nums[i].as_bytes()[i] ^ 1))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::vector_string::vector_string;

    #[test]
    fn test_find_different_binary_string_1() {
        let input = vector_string(vec!["01", "10"]);
        let result = Solution::find_different_binary_string(input);
        assert_eq!(result, "11".to_string());
    }

    // leetcode test case uses 11 as the default
    #[test]
    fn test_find_different_binary_string_2() {
        let input = vector_string(vec!["00", "01"]);
        let result = Solution::find_different_binary_string(input);
        assert_eq!(result, "10".to_string());
    }

    // leetcode test case uses 101 as the default
    #[test]
    fn test_find_different_binary_string_3() {
        let input = vector_string(vec!["111", "011", "001"]);
        let result = Solution::find_different_binary_string(input);
        assert_eq!(result, "000".to_string());
    }
}
