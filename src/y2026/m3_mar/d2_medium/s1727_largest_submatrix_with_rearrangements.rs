// LeetCode Problem 1727. Largest Submatrix With Rearrangements
// Difficulty: Medium
//
// Time Complexity: O(m * n) - where m is the number of rows and n is the number
// of cols in the matrix respectively
// Space Complexity: O(1) - auxiliary space, no additional data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 1..m {
            (0..n).for_each(|j| {
                if matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i - 1][j];
                }
            });
        }

        let mut ans = 0;

        (0..m).for_each(|i| {
            let mut row = matrix[i].clone();
            row.sort_unstable_by(|a, b| b.cmp(a));
            let tmp = 0..n;
            for j in tmp {
                if row[j] == 0 {
                    break;
                }
                ans = ans.max(row[j] * (j as i32 + 1));
            }
        });

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_submatrix_1() {
        let input = vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]];
        let result = Solution::largest_submatrix(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_largest_submatrix_2() {
        let input = vec![vec![1, 0, 1, 0, 1]];
        let result = Solution::largest_submatrix(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_largest_submatrix_3() {
        let input = vec![vec![1, 1, 0], vec![1, 0, 1]];
        let result = Solution::largest_submatrix(input);
        assert_eq!(result, 2);
    }
}
