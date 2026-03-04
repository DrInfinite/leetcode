// LeetCode Problem 1582. Special Positions in a Binary Matrix
// Difficulty: Easy
//
// Time Complexity: O(m * n) - where m is the length of the matrix, n is the length of each row in
// the matrix
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut row = vec![0; m];
        let mut col = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 && row[i] == 1 && col[j] == 1 {
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_special_test_1() {
        let input = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
        let result = Solution::num_special(input);
        assert_eq!(result, 1)
    }

    #[test]
    fn test_num_special_test_2() {
        let input = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let result = Solution::num_special(input);
        assert_eq!(result, 3)
    }
}
