// LeetCode Problem 3600. Maximize Spanning Tree Stability with Upgrades
// Difficulty: Hard

// Time Complexity: O(m log m) - where m is the size of the vector edges
// Space Complexity: O(n + m) - where m is the size of the vector edges, n is the size of
// an vector that is allocated

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_stability(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut f: Vec<usize> = (0..n as usize).collect();
        fn find(f: &mut [usize], mut x: usize) -> usize {
            while x != f[x] {
                f[x] = f[f[x]];
                x = f[x];
            }
            x
        }
        let mut cnt = n - 1;
        let mut ret = i32::MAX;
        edges.sort_unstable_by_key(|e| (-e[3], -e[2]));

        for e in &edges {
            if cnt == 0 && e[3] == 0 {
                break;
            }
            let u = find(&mut f, e[0] as usize);
            let v = find(&mut f, e[1] as usize);
            if u == v {
                if e[3] == 1 {
                    return -1;
                }
                continue;
            }
            f[u] = v;

            let val = if e[3] == 0 && cnt <= k {
                e[2] * 2
            } else {
                e[2]
            };
            ret = ret.min(val);
            cnt -= 1;
        }

        if cnt != 0 { -1 } else { ret }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_stability_1() {
        let input = vec![vec![0, 1, 2, 1], vec![1, 2, 3, 0]];
        let result = Solution::max_stability(3, input, 1);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_max_stability_2() {
        let input = vec![vec![0, 1, 4, 0], vec![1, 2, 3, 0], vec![0, 2, 1, 0]];
        let result = Solution::max_stability(3, input, 2);
        assert_eq!(result, 6)
    }

    #[test]
    fn test_max_stability_3() {
        let input = vec![vec![0, 1, 1, 1], vec![1, 2, 1, 1], vec![2, 0, 1, 1]];
        let result = Solution::max_stability(3, input, 0);
        assert_eq!(result, -1)
    }
}
