// LeetCode Problem 85. Maximal Rectangle
// Difficulty: Hard

// Time Complexity: O(R * C²) — For each of the R rows, the algorithm iterates
// over all C columns and may scan backward up to C columns to compute the
// maximum rectangle ending at each column.

// Space Complexity: O(C) — Uses a single DP array of size equal to the number
// of columns to store heights of consecutive '1's.

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let cols = matrix[0].len();
        let mut dp = vec![0usize; cols];

        matrix
            .iter()
            .map(|row| {
                (0..cols)
                    .map(|col| {
                        dp[col] = if row[col] == '1' { dp[col] + 1 } else { 0usize };
                        let mut res = dp[col];
                        let mut min_dp = dp[col];
                        for br in (0..col).rev() {
                            min_dp = min_dp.min(dp[br]);
                            res = res.max((col - br + 1) * min_dp);
                        }
                        res as i32
                    })
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximal_rectangle_1() {
        let input = vec![
            vec!['1', '0', '1', '0', 's'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        let result = Solution::maximal_rectangle(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_maximal_rectangle_2() {
        let input = vec![vec!['0']];
        let result = Solution::maximal_rectangle(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_maximal_rectangle_3() {
        let input = vec![vec!['1']];
        let result = Solution::maximal_rectangle(input);
        assert_eq!(result, 1);
    }
}
