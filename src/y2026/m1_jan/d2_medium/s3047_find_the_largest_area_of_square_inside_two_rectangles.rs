// LeetCode Problem 3047. Find the Largest Area of Square Inside Two Rectangles
// Difficulty: Medium

// Time Complexity: O(N²) - N is the number of rectangles; the algorithm checks
// every pair of rectangles once.

// Space Complexity: O(1) - Only constant extra space is used; no additional
// data structures grow with N.

#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        bottom_left[..bottom_left.len() - 1]
            .iter()
            .zip(&top_right)
            .enumerate()
            .flat_map(|(i, (ab, at))| {
                bottom_left[i + 1..]
                    .iter()
                    .zip(&top_right[i + 1..])
                    .map(|(bb, bt)| {
                        let xo = bt[0].min(at[0]) - bb[0].max(ab[0]);
                        let yo = bt[1].min(at[1]) - bb[1].max(ab[1]);
                        i64::from(xo.min(yo).max(0)).pow(2)
                    })
            })
            .max()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_square_area_1() {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![3, 1]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![6, 6]];
        let result = Solution::largest_square_area(bottom_left, top_right);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_largest_square_area_2() {
        let bottom_left = vec![vec![1, 1], vec![1, 3], vec![1, 5]];
        let top_right = vec![vec![5, 5], vec![5, 7], vec![5, 9]];
        let result = Solution::largest_square_area(bottom_left, top_right);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_largest_square_area_3() {
        let bottom_left = vec![vec![1, 1], vec![2, 2], vec![1, 2]];
        let top_right = vec![vec![3, 3], vec![4, 4], vec![3, 4]];
        let result = Solution::largest_square_area(bottom_left, top_right);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_largest_square_area_4() {
        let bottom_left = vec![vec![1, 1], vec![3, 3], vec![3, 1]];
        let top_right = vec![vec![2, 2], vec![4, 4], vec![4, 2]];
        let result = Solution::largest_square_area(bottom_left, top_right);
        assert_eq!(result, 0);
    }
}
