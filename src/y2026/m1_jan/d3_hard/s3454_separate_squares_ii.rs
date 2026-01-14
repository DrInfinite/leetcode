// LeetCode Problem 3454. Separate Squares II
// Difficulty: Hard

// Time Complexity: O(n² log n) — there are 2n sweep events, and at each y-level
// the active x-intervals can be re-sorted (up to O(n log n)) across O(n) levels.

// Space Complexity: O(n) — stores events, active intervals, and sweep-line segments,
// all linear in the number of squares.

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn separate_squares(s: Vec<Vec<i32>>) -> f64 {
        let mut ev: Vec<_> = s
            .iter()
            .flat_map(|v| {
                let (x, y, l) = (v[0] as f64, v[1] as f64, v[2] as f64);
                [(y, 1, x, x + l), (y + l, -1, x, x + l)]
            })
            .collect();
        ev.sort_by(|a, b| a.0.total_cmp(&b.0));

        let (mut xlr, mut sl, mut tot, mut py) = (vec![], vec![], 0., 0.);

        for (y, op, l, r) in ev {
            if y > py {
                xlr.sort_by(|a: &(f64, f64), b| a.0.total_cmp(&b.0));
                let (mut w, mut mx) = (0., 0.);
                for &(al, ar) in &xlr {
                    if mx < ar {
                        w += ar - mx.max(al);
                        mx = ar
                    }
                }
                sl.push((tot, py, y - py, w));
                tot += (y - py) * w;
                py = y
            }
            if op > 0 {
                xlr.push((l, r));
            } else {
                xlr.remove(xlr.iter().position(|&x| x == (l, r)).unwrap());
            }
        }

        let (cur, y, _, w) = sl
            .into_iter()
            .find(|&(c, _, h, w)| c + h * w >= tot / 2.)
            .unwrap();

        (tot / 2. - cur) / w + y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_separate_squares_1() {
        let input = vec![vec![0, 0, 1], vec![2, 2, 1]];
        let result = Solution::separate_squares(input);
        println!("{}", result);
        assert_eq!(result, 1.00000);
    }

    #[test]
    fn test_separate_squares_2() {
        let input = vec![vec![0, 0, 2], vec![1, 1, 1]];
        let result = Solution::separate_squares(input);
        println!("{}", result);
        assert_eq!(result, 1.00000);
    }
}
