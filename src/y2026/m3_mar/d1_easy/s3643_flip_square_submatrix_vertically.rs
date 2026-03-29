// LeetCode Problem 3643. Flip Square Submatrix Vertically
// Difficulty: Easy
//
// Time Complexity: O(k^2) - where k is the size of the square matrix
// Space Complexity: O(k^2) - where k is the size of the square matrix

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;

        let from_top = x..x + k / 2; // Top to down iterator
        let from_bottom = (x + k / 2..x + k).rev(); // Bottom to top

        for (top, bottom) in from_top.zip(from_bottom) {
            if top == bottom {
                // skip middle row
                break;
            }
            (y..y + k).for_each(|x| {
                let temp = grid[top][x];
                grid[top][x] = grid[bottom][x];
                grid[bottom][x] = temp;
            });
        }
        grid
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_reverse_submatrix_1() {
        let input = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .to_nested_vec();
        let result = Solution::reverse_submatrix(input, 1, 0, 3);
        let expected = [
            [1, 2, 3, 4],
            [13, 14, 15, 8],
            [9, 10, 11, 12],
            [5, 6, 7, 16],
        ]
        .to_nested_vec();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_reverse_submatrix_2() {
        let input = [[3, 4, 2, 3], [2, 3, 4, 2]].to_nested_vec();
        let result = Solution::reverse_submatrix(input, 0, 2, 2);
        let expected = [[3, 4, 4, 2], [2, 3, 2, 3]].to_nested_vec();

        assert_eq!(result, expected);
    }
}
