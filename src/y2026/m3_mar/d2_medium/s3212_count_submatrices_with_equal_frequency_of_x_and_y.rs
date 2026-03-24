// LeetCode Problem 3212. Count Submatrices With Equal Frequency of X and Y
// Difficulty: Medium
//
// Time Complexity: O(m * n) - where n is the number of rows and m is the number
// of cols in the grid respectively
// Space Complexity: O(m) - where m is the number of cols in the grid

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let m = grid[0].len();

        let mut col_sums = vec![(0, 0); m];
        let mut res = 0;

        for row in grid {
            let mut row_x = 0;
            let mut row_y = 0;

            for (col_sum, cell) in col_sums.iter_mut().zip(row) {
                match cell {
                    'X' => row_x += 1,
                    'Y' => row_y += 1,
                    _ => {}
                }
                col_sum.0 += row_x;
                col_sum.1 += row_y;

                if col_sum.0 > 0 && col_sum.0 == col_sum.1 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_submatrices_1() {
        let input = vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']];
        let result = Solution::number_of_submatrices(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_number_of_submatrices_2() {
        let input = vec![vec!['X', 'X'], vec!['X', 'Y']];
        let result = Solution::number_of_submatrices(input);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_number_of_submatrices_3() {
        let input = vec![vec!['.', '.'], vec!['.', '.']];
        let result = Solution::number_of_submatrices(input);

        assert_eq!(result, 0);
    }
}
