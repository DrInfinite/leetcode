// LeetCode Problem 1653. Minimum Deletions to Make String Balanced
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the length of the string s
// Space Complexity: O(1) - no additional allocations are made apart from s.

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        s.bytes()
            .skip_while(|&b| b == b'a')
            .fold((0, 0), |(x, y), b| match b {
                b'b' => (x, y + 1),
                _ => (y.min(x + 1), y),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_deletions_1() {
        let result = Solution::minimum_deletions("aababbab".to_string());
        assert_eq!(result, 2)
    }

    #[test]
    fn test_minimum_deletions_2() {
        let result = Solution::minimum_deletions("bbaaaaab".to_string());
        assert_eq!(result, 2)
    }
}
