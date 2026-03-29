// LeetCode Problem 2906. Construct Product Matrix
// Difficulty: Medium
//
// Time Complexity: O(m * n) - where m is the number of rows, n is the number of
// cols in the grid
// Space Complexity: O(m * n) - where m is the number of rows, n is the number of
// cols in the grid

#![allow(dead_code)]
struct Solution;

const MOD: i64 = 12345;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let lenx = grid[0].len();

        let mut prefix = [1]
            .into_iter()
            .chain(grid.iter().flatten().scan(1, |acc, &x| {
                *acc = (*acc * x as i64) % MOD;
                Some(*acc)
            }))
            .collect::<Vec<i64>>();
        prefix.pop();

        let mut postfix = [1]
            .into_iter()
            .chain(grid.into_iter().flatten().rev().scan(1, |acc, x| {
                *acc = (*acc * x as i64) % MOD;
                Some(*acc)
            }))
            .collect::<Vec<i64>>();
        postfix.pop();

        prefix
            .iter()
            .zip(postfix.iter().rev())
            .map(|(a, b)| ((a * b) % MOD) as i32)
            .collect::<Vec<i32>>()
            .chunks(lenx)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_construct_product_matrix_1() {
        let input = [[1, 2], [3, 4]].to_nested_vec();
        let result = Solution::construct_product_matrix(input);
        let expected = [[24, 12], [8, 6]].to_nested_vec();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_construct_product_matrix_2() {
        let input = [[12345], [2], [1]].to_nested_vec();
        let result = Solution::construct_product_matrix(input);
        let expected = [[2], [0], [0]].to_nested_vec();

        assert_eq!(result, expected);
    }
}
