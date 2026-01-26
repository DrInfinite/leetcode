// LeetCode Problem 3453. Separate Squares I
// Difficulty: Medium

// Time Complexity: O(n) — one linear scan plus a constant (80) binary search iterations over all squares
// Space Complexity: O(1) — only constant extra variables are used

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut lo = 1e18f64;
        let mut hi = -1e18f64;
        let mut total = 0.00000f64;

        for s in &squares {
            let y = s[1] as f64;
            let l = s[2] as f64;
            if y < lo {
                lo = y;
            }
            if y + l > hi {
                hi = y + l;
            }
            total += l * l;
        }

        let target = total / 2.00000;

        for _ in 0..80 {
            let mid = (lo + hi) * 0.50000;
            let mut below = 0.00000f64;
            for s in &squares {
                let y = s[1] as f64;
                let l = s[2] as f64;
                if mid > y {
                    let h = (mid - y).min(l);
                    below += h * l;
                }
            }
            if below < target {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        ((lo * 100000.0).round() / 100000.0) as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_separate_squares_1() {
        let input = vec![vec![0, 0, 1], vec![2, 2, 1]];
        let result = Solution::separate_squares(input);
        assert_eq!(1.00000, result);
    }

    #[test]
    fn test_separate_squares_2() {
        let input = vec![vec![0, 0, 2], vec![1, 1, 1]];
        let result = Solution::separate_squares(input);
        assert_eq!(1.16667, result);
    }
}
