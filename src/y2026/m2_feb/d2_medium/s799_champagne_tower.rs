// LeetCode Problem 799. Champagne Tower
// Difficulty: Medium

// Time Complexity: O(n^2) - where n is the size of vector `row`
// Space Complexity: O(n) - where n is the size of vector `row`

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut row = vec![poured as f64];

        for _ in 0..query_row as usize {
            row.push(0.0);

            for j in (0..row.len()).rev().skip(1) {
                row[j] = if row[j] > 1.0 { row[j] - 1.0 } else { 0.0 };
                row[j] /= 2.0;
                row[j + 1] += row[j];
            }
        }

        row[query_glass as usize].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_champagne_tower_1() {
        let result = Solution::champagne_tower(1, 1, 1);
        assert_eq!(result, 0.00000)
    }

    #[test]
    fn test_champagne_tower_2() {
        let result = Solution::champagne_tower(2, 1, 1);
        assert_eq!(result, 0.50000)
    }

    #[test]
    fn test_champagne_tower_3() {
        let result = Solution::champagne_tower(100000009, 33, 17);
        assert_eq!(result, 1.00000)
    }
}
