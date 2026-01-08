// LeetCode Problem Q2. Find the Pivot Integer
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the array
// Space Complexity: O(n) - where n is the number of elements in the array

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total = n * (n + 1) / 2;
        let x = (total as f64).sqrt() as i32;

        if x * x == total { x } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_integer_1() {
        assert_eq!(Solution::pivot_integer(8), 6);
    }

    #[test]
    fn test_pivot_integer_2() {
        assert_eq!(Solution::pivot_integer(1), 1);
    }

    #[test]
    fn test_pivot_integer_3() {
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
