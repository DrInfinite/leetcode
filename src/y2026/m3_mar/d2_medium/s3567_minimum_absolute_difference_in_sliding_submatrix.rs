// LeetCode Problem 3567. Minimum Absolute Difference in Sliding Submatrix
// Difficulty: Medium

// Time Complexity: O(n m k^2 log k) - where n is the number of rows in the grid,
// m is the number of cols in the grid, k is the square subgrid

// Space Complexity: O (n m k^2) - where n is the number of rows in the grid,
// m is the number of cols in the grid, k is the square subgrid

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let k: usize = usize::try_from(k).unwrap();
        (0..=n - k)
            .map(|i| {
                (0..=m - k)
                    .map(|j| {
                        let mut subgrid: Vec<i32> = grid
                            .iter()
                            .skip(i)
                            .take(k)
                            .flat_map(|row| row.iter().skip(j).take(k).copied())
                            .collect();
                        subgrid.sort_unstable();
                        subgrid.dedup();
                        subgrid.windows(2).map(|x| x[1] - x[0]).min().unwrap_or(0)
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_min_abs_diff_1() {
        let input = [[1, 8], [3, -2]].to_nested_vec();
        let result = Solution::min_abs_diff(input, 2);
        let expected = [[2]].to_nested_vec();

        assert_eq!(result, expected)
    }

    #[test]
    fn test_min_abs_diff_2() {
        let input = [[3, -1]].to_nested_vec();
        let result = Solution::min_abs_diff(input, 1);
        let expected = [[0, 0]].to_nested_vec();

        assert_eq!(result, expected)
    }

    #[test]
    fn test_min_abs_diff_3() {
        let input = [[1, -2, 3], [2, 3, 5]].to_nested_vec();
        let result = Solution::min_abs_diff(input, 2);
        let expected = [[1, 2]].to_nested_vec();

        assert_eq!(result, expected)
    }
}
