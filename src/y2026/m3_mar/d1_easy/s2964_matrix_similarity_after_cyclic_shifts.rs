// LeetCode Problem 2946. Matrix Similarity After Cyclic Shifts
// Difficulty: Easy
//
// Time Complexity: O(m * n) - where m is the length of the matrix, n is the length of each row in
// the matrix
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        mat.iter().all(|row| {
            row.iter()
                .zip(row.iter().chain(row.iter()).skip(k as usize % mat[0].len()))
                .all(|(a, b)| a == b)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_are_similar_1() {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].to_nested_vec();
        let result = Solution::are_similar(input, 4);

        assert!(!result);
    }

    #[test]
    fn test_are_similar_2() {
        let input = [[1, 2, 1, 2], [5, 5, 5, 5], [6, 3, 6, 3]].to_nested_vec();
        let result = Solution::are_similar(input, 2);

        assert!(result);
    }

    #[test]
    fn test_are_similar_3() {
        let input = [[2, 2], [2, 2]].to_nested_vec();
        let result = Solution::are_similar(input, 3);

        assert!(result);
    }
}
