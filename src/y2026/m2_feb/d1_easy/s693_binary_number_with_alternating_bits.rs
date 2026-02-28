// LeetCode Problem 693. Binary Number with Alternating Bits
// Difficulty: Easy
//
// Time Complexity: O(1) - constant time, max value is a 32-bit integer
// Space Complexity: O(1) - constant memory, no additional allocations

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        n ^= n / 2;
        n & n + 1 == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_alternating_bits_1() {
        let result = Solution::has_alternating_bits(5);
        assert!(result);
    }

    #[test]
    fn test_has_alternating_bits_2() {
        let result = Solution::has_alternating_bits(7);
        assert!(!result);
    }

    #[test]
    fn test_has_alternating_bits_3() {
        let result = Solution::has_alternating_bits(11);
        assert!(!result);
    }
}
