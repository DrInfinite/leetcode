// LeetCode Problem 66. Plus One
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the array
// Space Complexity: O(1) - only using a constant amount of extra space

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            if *digit == 9 {
                *digit = 0
            } else {
                *digit += 1;
                return digits;
            }
        }
        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one_simple() {
        let digits = vec![1, 2, 3];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1, 2, 4]);
    }

    #[test]
    fn test_plus_one_with_carry_at_end() {
        let digits = vec![4, 3, 2, 1];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_plus_one_all_nines() {
        let digits = vec![9];
        let result = Solution::plus_one(digits);
        assert_eq!(result, vec![1, 0]);
    }
}
