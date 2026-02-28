// LeetCode Problem 190. Reverse Bits
// Difficulty: Easy
//
// Time Complexity: O(1) - constant time of 32 bits
// Space Complexity: O(1) - constant space of 32 bits

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        n.reverse_bits()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_bits_1() {
        let result = Solution::reverse_bits(43261596);
        assert_eq!(result, 964176192)
    }

    #[test]
    fn test_reverse_bits_2() {
        let result = Solution::reverse_bits(2147483644);
        assert_eq!(result, 1073741822)
    }
}
