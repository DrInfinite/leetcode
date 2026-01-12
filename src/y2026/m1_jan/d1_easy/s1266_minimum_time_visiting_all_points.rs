// LeetCode Problem 1266. Minimum Time Visiting All Points
// Difficulty: Easy
//
// Time Complexity: O(n) - single pass through the points
// Space Complexity: O(1) - constant extra space

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;

        for i in 1..points.len() {
            let max_w = (points[i][0] - points[i - 1][0]).abs();
            let max_h = (points[i][1] - points[i - 1][1]).abs();
            res += max_h.max(max_w);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one_simple() {
        let input = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
        let result = Solution::min_time_to_visit_all_points(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_plus_one_with_carry_at_end() {
        let input = vec![vec![3, 2], vec![-2, 2]];
        let result = Solution::min_time_to_visit_all_points(input);
        assert_eq!(result, 5);
    }
}
