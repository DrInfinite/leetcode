// LeetCode Problem 1758. Minimum Changes To Make Alternating Binary String
// Difficulty: Easy
//
// Time Complexity: o(n) - where n is the length of the string
// Space Complexity: O(1) - Constant space, no addition data structures allocated

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        s.bytes()
            .zip((0..2).cycle())
            .fold([0, 0], |mut zo, (b, x)| {
                zo[(b & 1 ^ x) as usize] += 1;
                zo
            })
            .into_iter()
            .fold(i32::MAX, i32::min)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_operations_1() {
        let result = Solution::min_operations("0100".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_min_operations_2() {
        let result = Solution::min_operations("10".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_min_operations_3() {
        let result = Solution::min_operations("1111".to_string());
        assert_eq!(result, 2);
    }
}
