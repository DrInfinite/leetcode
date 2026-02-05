// LeetCode Problem 3379. Transformed Array
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the size of the array nums
// Space Complexity: O(n) - where n is the size of the array result

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut result = vec![];

        for i in 0..n {
            let idx = ((i as i32) + nums[i]).rem_euclid(n as i32) as usize;
            result.push(nums[idx]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_transformed_array_1() {
        let input = vec![3, -2, 1, 1];
        let result = Solution::construct_transformed_array(input);
        let expected = vec![1, 1, 1, 3];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_construct_transformed_array_2() {
        let input = vec![-1, 4, -1];
        let result = Solution::construct_transformed_array(input);
        let expected = vec![-1, -1, 4];
        assert_eq!(result, expected)
    }
}
