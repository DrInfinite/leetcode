// LeetCode Problem 1536. Minimum Swaps to Arrange a Binary Grid
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of the matrix grid
// Space Complexity: O(1) - constant space, no new data-structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_swaps(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut swaps = 0;

        for i in 1..grid.len() {
            let Some(p) = grid.iter().position(|row| row[i..].iter().all(|&b| b == 0)) else {
                return -1;
            };

            grid.remove(p);
            swaps += p as i32;
        }

        swaps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_swaps_1() {
        let input = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
        let result = Solution::min_swaps(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_min_swaps_2() {
        let input = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
        ];
        let result = Solution::min_swaps(input);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_min_swaps_3() {
        let input = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]];
        let result = Solution::min_swaps(input);
        assert_eq!(result, 0);
    }
}
