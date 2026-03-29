// LeetCode Problem 1594. Maximum Non Negative Product in a Matrix
// Difficulty: Medium
//
// Time Complexity: O(m * n) - where m is the number of rows, n is the number of
// cols in the grid;
// Space Complexity: O(n) - where n is the number of cols

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();

        let mut dp = vec![(i64::MAX, i64::MIN); n + 1];
        dp[0] = (1, 1);

        for row in grid.iter() {
            for (j, &val) in row.iter().enumerate() {
                let (a, b) = (dp[j].0 * val as i64, dp[j].1 * val as i64);
                let (min, max) = (a.min(b), a.max(b));

                dp[j] = (min, max);
                dp[j + 1].0 = dp[j + 1].0.min(min);
                dp[j + 1].1 = dp[j + 1].1.max(max);
            }
        }

        (dp[n - 1].1.max(-1) % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_max_product_path_1() {
        let input = [[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]].to_nested_vec();
        let result = Solution::max_product_path(input);

        assert_eq!(result, -1);
    }

    #[test]
    fn test_max_product_path_2() {
        let input = [[1, -2, 1], [1, -2, 1], [3, -4, 1]].to_nested_vec();
        let result = Solution::max_product_path(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_max_product_path_3() {
        let input = [[1, 3], [0, -4]].to_nested_vec();
        let result = Solution::max_product_path(input);

        assert_eq!(result, 0);
    }
}
