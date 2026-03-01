// LeetCode Problem 1404. Number of Steps to Reduce a Number in Binary Representation to One
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of the vector arr
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        s.bytes()
            .skip(1)
            .rev()
            .fold([0, 0], |[s, c], b| match i32::from(b & 1) + c {
                1 => [s + 2, 1],
                _ => [s + 1, c],
            })
            .into_iter()
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_steps_1() {
        let result = Solution::num_steps("1101".to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn test_num_steps_2() {
        let result = Solution::num_steps("10".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_num_steps_3() {
        let result = Solution::num_steps("1".to_string());
        assert_eq!(result, 0);
    }
}
