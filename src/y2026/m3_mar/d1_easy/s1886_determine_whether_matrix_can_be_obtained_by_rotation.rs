// LeetCode Problem 1886. Determine Whether Matrix Can Be Obtained By Rotation
// Difficulty: Easy
//
// Time Complexity: O(n^2) - where n is the size of the square matrix
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        let [h, x] = [n / 2, !n & 1];
        (0..4).any(|t| {
            (0..n).all(|i| {
                (0..n).all(|j| {
                    let [oi, oj] = (0..t).fold([i, j], |[i, j], _| [h + h - j - x, i]);
                    mat[oi][oj] == target[i][j]
                })
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_find_rotation_1() {
        let mat = [[0, 1], [1, 0]].to_nested_vec();
        let target = [[1, 0], [0, 1]].to_nested_vec();
        let result = Solution::find_rotation(mat, target);

        assert!(result);
    }

    #[test]
    fn test_find_rotation_2() {
        let mat = [[0, 1], [1, 1]].to_nested_vec();
        let target = [[1, 0], [0, 1]].to_nested_vec();
        let result = Solution::find_rotation(mat, target);

        assert!(!result);
    }

    #[test]
    fn test_find_rotation_3() {
        let mat = [[0, 0, 0], [0, 1, 0], [1, 1, 1]].to_nested_vec();
        let target = [[1, 1, 1], [0, 1, 0], [0, 0, 0]].to_nested_vec();
        let result = Solution::find_rotation(mat, target);

        assert!(result);
    }
}
