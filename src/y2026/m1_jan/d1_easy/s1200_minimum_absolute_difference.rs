// LeetCode Problem 1200. Minimum Absolute Difference
// Difficulty: Easy
//
// Time Complexity: O(n * log n) - sorting the array dominates the linear scans
// Space Complexity: O(n) - sorting is in-place and only the result is stored

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let min = arr.windows(2).map(|w| w[1] - w[0]).min().unwrap_or(0);
        arr.windows(2)
            .filter(|w| w[1] - w[0] == min)
            .map(|w| vec![w[0], w[1]])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_abs_difference_1() {
        let input = vec![4, 2, 1, 3];
        let result = Solution::minimum_abs_difference(input);
        let expected = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_minimum_abs_difference_2() {
        let input = vec![1, 3, 6, 10, 15];
        let result = Solution::minimum_abs_difference(input);
        let expected = vec![vec![1, 3]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_minimum_abs_difference_3() {
        let input = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let result = Solution::minimum_abs_difference(input);
        let expected = vec![vec![-14, -10], vec![19, 23], vec![23, 27]];
        assert_eq!(result, expected);
    }
}
