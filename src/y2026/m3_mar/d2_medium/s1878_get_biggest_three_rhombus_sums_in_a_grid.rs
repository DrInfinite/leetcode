// LeetCode Problem 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
// Difficulty: Medium
//
// Time Complexity: O(mn * min(m,n) ^ 2) - where m is the number of rows and n is
// the number sof cols in the grid. 
// Space Complexity: O(1) - constant space or auxiliary space.

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut top3: Vec<i32> = Vec::new();

        let mut add_to_top3 = |val: i32| {
            if !top3.contains(&val) {
                top3.push(val);
                top3.sort_unstable_by(|a, b| b.cmp(a));
                if top3.len() > 3 {
                    top3.pop();
                }
            }
        };

        for i in 0..m {
            for j in 0..n {
                add_to_top3(grid[i][j]);

                let max_r = (i.min(m - 1 - i)).min(j.min(n - 1 - j));

                for r in 1..=max_r {
                    let mut sum = grid[i + r][j] + grid[i - r][j];
                    for k in 1..r {
                        sum += grid[i - k][j + r - k];
                        sum += grid[i + k][j + r - k];
                        sum += grid[i - k][j - r + k];
                        sum += grid[i + k][j - r + k];
                    }
                    sum += grid[i][j + r] + grid[i][j - r];
                    add_to_top3(sum);
                }
            }
        }

        top3
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_biggest_three_1() {
        let input = vec![
            vec![3, 4, 5, 1, 3], 
            vec![3, 3, 4, 2, 3], 
            vec![20, 30, 200, 40, 10], 
            vec![1, 5, 5, 4, 1], 
            vec![4, 3, 2, 2, 5],
        ];
        let result = Solution::get_biggest_three(input);
        let output = vec![228, 216, 211];
        assert_eq!(result, output);
    }

    #[test]
    fn test_get_biggest_three_2() {
        let input = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let result = Solution::get_biggest_three(input);
        let output = vec![20, 9, 8];
        assert_eq!(result, output);
    }

    #[test]
    fn test_get_biggest_three_3() {
        let input = vec![vec![7, 7, 7]];
        let result = Solution::get_biggest_three(input);
        let output = vec![7];
        assert_eq!(result, output);
    }
}