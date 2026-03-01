// LeetCode Problem 1680. Concatenation of Consecutive Binary Numbers
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the range of the iterator
// Space Complexity: O(1) - no additional allocations are made

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (2..=n).fold(1, |r, n| {
            (r << (32 - n.leading_zeros()) | n as u64) % 1_000_000_007
        }) as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_concatenated_binary_1() {
        let result = Solution::concatenated_binary(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_concatenated_binary_2() {
        let result = Solution::concatenated_binary(3);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_concatenated_binary_3() {
        let result = Solution::concatenated_binary(12);
        assert_eq!(result, 505379714);
    }
}
