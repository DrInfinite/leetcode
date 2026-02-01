// LeetCode Problem 3651. Minimum Cost Path with Teleportations
// Difficulty: Hard
//
// Time Complexity: O(k · (n · m + V))
//   where n = number of rows, m = number of columns,
//   k = number of teleportation relaxations,
//   V = maximum value in the grid.
//
//   Since grid cloning is removed, each iteration now performs
//   only DP propagation and relaxations over the grid.
//   If V is bounded by a constant, this simplifies to O(k · n · m).
//
// Space Complexity: O(n · m + V)
//   for the DP matrix (mat) and the min_steps array.
//   No extra O(n · m) space is used for grid cloning.

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut mat = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        mat[0][0] = 0;

        let grid_max = Solution::downright_propagation(&grid, &mut mat) as usize;

        for _ in 0..k {
            let mut min_steps = vec![i32::MAX; grid_max + 1];

            (0..grid.len()).for_each(|i| {
                (0..grid[0].len()).for_each(|j| {
                    min_steps[grid[i][j] as usize] = min_steps[grid[i][j] as usize].min(mat[i][j]);
                })
            });

            ((0..grid_max).rev()).for_each(|i| {
                min_steps[i] = min_steps[i].min(min_steps[i + 1]);
            });

            (0..grid.len()).for_each(|i| {
                (0..grid[0].len()).for_each(|j| {
                    mat[i][j] = mat[i][j].min(min_steps[grid[i][j] as usize]);
                })
            });

            Solution::downright_propagation(&grid, &mut mat);
        }

        *mat.last().unwrap().last().unwrap()
    }

    fn downright_propagation(grid: &Vec<Vec<i32>>, mat: &mut [Vec<i32>]) -> i32 {
        let mut grid_max = 0;

        (0..grid.len()).for_each(|i| {
            (0..grid[0].len()).for_each(|j| {
                grid_max = grid_max.max(grid[i][j]);

                if i > 0 && j > 0 {
                    mat[i][j] = mat[i][j].min(mat[i - 1][j].min(mat[i][j - 1]) + grid[i][j]);
                } else if i > 0 {
                    mat[i][j] = mat[i][j].min(mat[i - 1][j] + grid[i][j]);
                } else if j > 0 {
                    mat[i][j] = mat[i][j].min(mat[i][j - 1] + grid[i][j]);
                }
            })
        });

        grid_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_1() {
        let input = vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]];
        let result = Solution::min_cost(input, 2);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_min_cost_2() {
        let input = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        let result = Solution::min_cost(input, 1);

        assert_eq!(result, 9);
    }
}
