// LeetCode Problem 3546. Equal Sum Grid Partition I
// Difficulty: Medium

// Time Complexity: O(m * n) - where m is the number of rows, and n is the number
// of cols in the grid
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn can_partition_grid(g: Vec<Vec<i32>>) -> bool {
        let (mut h, mut v, t) = (0, 0, g.iter().flatten().fold(0, |s, &x| s + x as i64));

        g.iter().any(|r| {
            h += r.iter().sum::<i32>() as i64;
            h + h == t
        }) || (0..g[0].len()).any(|c| {
            v += g.iter().fold(0, |s, r| s + r[c] as i64);
            v + v == t
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_can_partition_grid_1() {
        let input = [[1, 4], [2, 3]].to_nested_vec();
        let result = Solution::can_partition_grid(input);

        assert!(result)
    }

    #[test]
    fn test_can_partition_grid_2() {
        let input = [[1, 3], [2, 4]].to_nested_vec();
        let result = Solution::can_partition_grid(input);

        assert!(!result)
    }
}
