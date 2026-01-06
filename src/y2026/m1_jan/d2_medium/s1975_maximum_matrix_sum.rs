// LeetCode Problem 1975. Maximum Matrix Sum
// Difficulty: Medium
//
// Time Complexity: O(r × c) — where r is the number of rows in the matrix and c is the number of columns.
// Space Complexity: O(1) - constant size where n is the size of the constant

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut min_value = i64::MAX;
        let mut sum = 0_i64;
        let mut neg_count = 0_i64;

        (0..matrix.len()).for_each(|i| {
            (0..matrix[0].len()).for_each(|j| {
                if matrix[i][j] < 0 {
                    neg_count += 1;
                }
                let abs_value = i32::abs(matrix[i][j]) as i64;
                min_value = i64::min(min_value, abs_value);
                sum += abs_value;
            });
        });

        if neg_count % 2 == 0 {
            return sum;
        }

        sum - 2 * min_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_matrix_sum_1() {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        let result = Solution::max_matrix_sum(matrix);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_max_matrix_sum_2() {
        let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
        let result = Solution::max_matrix_sum(matrix);
        assert_eq!(result, 16);
    }
}
