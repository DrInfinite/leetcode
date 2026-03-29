// LeetCode Problem 3548. Equal Sum Grid Partition II
// Difficulty: Medium

// Time Complexity: O(m * n) - where m is the number of rows, and n is the number
// of cols in the grid
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn can_partition_grid(g: Vec<Vec<i32>>) -> bool {
        let l: Vec<Vec<_>> = g
            .iter()
            .map(|r| r.iter().map(|&v| v as i64).collect())
            .collect();
        let t: i64 = l.iter().flatten().sum();
        let x: Vec<Vec<_>> = (0..l[0].len())
            .map(|c| l.iter().map(|r| r[c]).collect())
            .collect();
        let (mut lr, mut xr) = (l.clone(), x.clone());
        lr.reverse();
        xr.reverse();
        let f = |m: &Vec<Vec<i64>>| {
            let (r, c, mut p, mut map) = (m.len() - 1, m[0].len() - 1, 0, HashMap::new());
            (0..=r).any(|y| {
                m[y].iter().enumerate().any(|(x, &v)| {
                    let i = map.get(&((t - v) / 2));
                    p += v;
                    (t - v) % 2 == 0
                        && i.map_or_else(
                            || false,
                            |&i| {
                                if c < 1 {
                                    y == i + 1 || y == r
                                } else {
                                    i < r - 1 || x % c == 0
                                }
                            },
                        )
                }) || y < r && {
                    map.insert(p, y);
                    p * 2 == t
                }
            })
        };

        ![&l, &lr, &x, &xr].into_iter().all(|m| !f(m))
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
        let input = [[1, 2], [3, 4]].to_nested_vec();
        let result = Solution::can_partition_grid(input);

        assert!(result)
    }

    #[test]
    fn test_can_partition_grid_3() {
        let input = [[1, 2, 4], [2, 3, 5]].to_nested_vec();
        let result = Solution::can_partition_grid(input);

        assert!(!result)
    }
}
