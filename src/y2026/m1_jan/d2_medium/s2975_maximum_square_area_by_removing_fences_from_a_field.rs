// LeetCode Problem 2975. Maximum Square Area by Removing Fences From a Field
// Difficulty: Medium

// Time Complexity: O(H² + V²) - H is the number of horizontal fence positions
// including borders, and V is the number of vertical fence positions including borders.

// Space Complexity: O(H² + V²) - Stores all possible horizontal and vertical
// edge lengths, where H and V are defined as above.

#![allow(dead_code)]

use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let get_edges = |fences: &[i32], border: i32| -> HashSet<i32> {
            let mut points = vec![1];
            points.extend_from_slice(fences);
            points.push(border);
            points.sort_unstable();

            (0..points.len())
                .flat_map(|i| {
                    let pi = points[i];
                    points[i + 1..].iter().map(move |&b| b - pi)
                })
                .collect()
        };

        let h_edges = get_edges(&h_fences, m);
        let v_edges = get_edges(&v_fences, n);

        h_edges
            .intersection(&v_edges)
            .max()
            .map_or(-1, |&e| ((e as i64).pow(2) % MOD) as i32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximize_square_area_1() {
        let h_fences = vec![2, 3];
        let v_fences = vec![2];
        let result = Solution::maximize_square_area(4, 3, h_fences, v_fences);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_maximize_square_area_2() {
        let h_fences = vec![2];
        let v_fences = vec![4];
        let result = Solution::maximize_square_area(6, 7, h_fences, v_fences);
        assert_eq!(result, -1);
    }
}
