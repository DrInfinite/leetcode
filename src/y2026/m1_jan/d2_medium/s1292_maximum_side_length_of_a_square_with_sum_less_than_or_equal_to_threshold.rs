// LeetCode Problem 1292. Maximum Side Length of a Square with Sum Less than or Equal to Threshold
// Difficulty: Medium

// Time Complexity: O(m . n) - where m and n are teh number of rows and columns
// of the matrix, respectively

// Space Complexity: O(m . n) - where m and n are teh number of rows and columns
// of the matrix, respectively

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let suffix_sum_2d = Self::suffix_sum_2d(mat);

        let mut next_size = 1;
        for r in (0..m).rev() {
            for c in (0..=(n - next_size)).rev() {
                if Self::sub_mat_sum(&suffix_sum_2d, r, c, next_size) <= threshold {
                    next_size += 1;
                    break;
                }
            }

            if next_size > n {
                break;
            }
        }

        next_size as i32 - 1
    }

    fn suffix_sum_2d(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();

        let mut suffix_sum = mat;
        suffix_sum.push(vec![0; n + 1]);
        for r in (0..m).rev() {
            suffix_sum[r].push(0);
            for c in (0..n).rev() {
                suffix_sum[r][c] +=
                    suffix_sum[r][c + 1] + suffix_sum[r + 1][c] - suffix_sum[r + 1][c + 1]
            }
        }

        suffix_sum
    }

    fn sub_mat_sum(suffix_sum_2d: &[Vec<i32>], r: usize, c: usize, size: usize) -> i32 {
        suffix_sum_2d[r][c] - suffix_sum_2d[r + size][c] - suffix_sum_2d[r][c + size]
            + suffix_sum_2d[r + size][c + size]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_side_length_1() {
        let input = vec![
            vec![1, 1, 3, 2, 4, 3, 2],
            vec![1, 1, 3, 2, 4, 3, 2],
            vec![1, 1, 3, 2, 4, 3, 2],
        ];
        let result = Solution::max_side_length(input, 4);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_max_side_length_2() {
        let input = vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
        ];
        let result = Solution::max_side_length(input, 1);
        assert_eq!(result, 0);
    }
}
