// LeetCode Problem 1784. Check if Binary String Has at Most One Segment of Ones
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the string s
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        s.bytes().skip_while(|&b| b == b'1').all(|b| b == b'0')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_ones_segment_1() {
        let result = Solution::check_ones_segment("1001".to_string());
        assert!(!result);
    }

    #[test]
    fn test_check_ones_segment_2() {
        let result = Solution::check_ones_segment("110".to_string());
        assert!(result);
    }
}
