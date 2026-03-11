// LeetCode Problem 1009. Complement of Base 10 Integer
// Difficulty: Easy
//
// Time Complexity: O(1) - constant time, no loops are present
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let bits = (n as f64).log2() as u32 + 1;
        let mask = (1 << bits) - 1;

        n ^ mask
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitwise_complement_1() {
        let result = Solution::bitwise_complement(5);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_bitwise_complement_2() {
        let result = Solution::bitwise_complement(7);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bitwise_complement_3() {
        let result = Solution::bitwise_complement(10);
        assert_eq!(result, 5);
    }
}
