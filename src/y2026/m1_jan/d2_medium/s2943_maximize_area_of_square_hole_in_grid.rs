// LeetCode Problem 2943. Maximize Area of Square Hole in Grid
// Difficulty: Medium

// Time Complexity: O(H log H + V log V) - Both horizontal and vertical bars are
// sorted, which dominates the runtime.

// Space Complexity: O(1) - Sorting is done in-place using `sort_unstable`, and
// only constant extra space is used.

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(_n: i32, _m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        [h_bars, v_bars]
            .into_iter()
            .filter_map(|mut bars| {
                bars.sort_unstable();
                bars.chunk_by(|a, b| b - a == 1)
                    .map(|c| c.len() as i32 + 1)
                    .max()
            })
            .min()
            .unwrap()
            .pow(2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximize_square_hole_area_1() {
        let h_bars = vec![2, 3];
        let v_bars = vec![2];
        let result = Solution::maximize_square_hole_area(2, 1, h_bars, v_bars);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_maximize_square_hole_area_2() {
        let h_bars = vec![2];
        let v_bars = vec![2];
        let result = Solution::maximize_square_hole_area(1, 1, h_bars, v_bars);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_maximize_square_hole_area_3() {
        let h_bars = vec![2, 3];
        let v_bars = vec![2, 4];
        let result = Solution::maximize_square_hole_area(2, 3, h_bars, v_bars);
        assert_eq!(result, 4);
    }
}
