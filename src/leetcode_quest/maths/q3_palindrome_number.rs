// LeetCode Problem Q3. Palindrome Number
// Difficulty: Easy
//
// Time Complexity: O(log n) - where n is the length of number n
// Space Complexity: O(1) - no new data structure created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut num = x;
        let mut reversed = 0;

        while num != 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }

        x == reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_1() {
        let result = Solution::is_palindrome(121);
        assert!(result);
    }

    #[test]
    fn test_is_palindrome_2() {
        let result = Solution::is_palindrome(-121);
        assert!(!result);
    }

    #[test]
    fn test_is_palindrome_3() {
        let result = Solution::is_palindrome(10);
        assert!(!result);
    }
}
