// LeetCode Problem Q1. Can Make Arithmetic Progression From Sequence
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the array
// Space Complexity: O(n) - where n is the number of elements in the array

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        if arr.len() <= 2 {
            return true;
        }

        arr.sort();

        let diff = arr[1] - arr[0];

        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != diff {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_make_arithmetic_progression_1() {
        let arr = vec![3, 5, 1];
        assert!(Solution::can_make_arithmetic_progression(arr));
    }

    #[test]
    fn test_can_make_arithmetic_progression_2() {
        let arr = vec![1, 2, 4];
        assert!(!Solution::can_make_arithmetic_progression(arr));
    }
}
