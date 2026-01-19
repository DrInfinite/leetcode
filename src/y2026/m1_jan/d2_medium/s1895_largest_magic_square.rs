// LeetCode Problem 1895. Largest Magic Square
// Difficulty: Medium
//
// Time Complexity: O(m . n . min(m, n)^2), where m is the number of rows and n is the number of
// columns
// Space Complexity: O(m . n), where m is the number of rows and n is the number of
// columns

#![allow(dead_code)]

use std::cmp::{max, min};
struct Solution;

impl Solution {
    fn is_magic(
        r: usize,
        c: usize,
        k: usize,
        row_prefix: &[Vec<i32>],
        col_prefix: &[Vec<i32>],
        main_diag: &[Vec<i32>],
        anti_diag: &[Vec<i32>],
    ) -> bool {
        if k == 1 {
            return true;
        };

        let target = row_prefix[r][c + k] - row_prefix[r][c];

        let i_tmp = (r + 1)..(r + k);
        for i in i_tmp {
            let sum = row_prefix[i][c + k] - row_prefix[i][c];
            if sum != target {
                return false;
            }
        }

        let j_tmp = c..(c + k);
        for j in j_tmp {
            let sum = col_prefix[r + k][j] - col_prefix[r][j];

            if sum != target {
                return false;
            }
        }

        let main_sum = main_diag[r + k][c + k] - main_diag[r][c];
        if main_sum != target {
            return false;
        }

        let anti_sum = anti_diag[r + k][c] - anti_diag[r][c + k];
        if anti_sum != target {
            return false;
        }

        true
    }

    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        // Initialisation
        let len_m = grid.len();
        let len_n = grid[0].len();

        // Compute prefix sums for each row and column
        let mut row_prefix = vec![vec![0; len_n + 1]; len_m];
        let mut col_prefix = vec![vec![0; len_n]; len_m + 1];

        for i in 0..len_m {
            for j in 0..len_n {
                row_prefix[i][j + 1] = row_prefix[i][j] + grid[i][j];
            }
        }

        for j in 0..len_n {
            for i in 0..len_m {
                col_prefix[i + 1][j] = col_prefix[i][j] + grid[i][j];
            }
        }

        // Compute prefix sum for each diagonal
        let mut main_diag = vec![vec![0; len_n + 1]; len_m + 1];
        let mut anti_diag = vec![vec![0; len_n + 1]; len_m + 1];

        for i in 0..len_m {
            for j in 0..len_n {
                main_diag[i + 1][j + 1] = main_diag[i][j] + grid[i][j];
                anti_diag[i + 1][j] = anti_diag[i][j + 1] + grid[i][j];
            }
        }

        // Searching for the largest magic square
        let mut max_k: usize = 1;

        for i in 0..len_m {
            for j in 0..len_n {
                let max_possible_k = min(len_m - i, len_n - j);

                for k in ((max_k + 1)..=max_possible_k).rev() {
                    if Solution::is_magic(i, j, k, &row_prefix, &col_prefix, &main_diag, &anti_diag)
                    {
                        max_k = max(max_k, k);
                        break;
                    }
                }
            }
        }

        max_k as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_magic_square_1() {
        let input = vec![
            vec![7, 1, 4, 5, 6],
            vec![2, 5, 1, 6, 4],
            vec![1, 5, 4, 3, 2],
            vec![1, 2, 7, 3, 4],
        ];
        let result = Solution::largest_magic_square(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_largest_magic_square_2() {
        let input = vec![vec![5, 1, 3, 1], vec![9, 3, 3, 1], vec![1, 3, 3, 8]];
        let result = Solution::largest_magic_square(input);
        assert_eq!(result, 2);
    }
}
