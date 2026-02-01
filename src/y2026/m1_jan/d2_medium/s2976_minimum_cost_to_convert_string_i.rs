// LeetCode Problem 2975. Maximum Square Area by Removing Fences From a Field
// Difficulty: Medium

// Time Complexity: O(m + n) (constant-sized Floyd–Warshall dominates but is fixed)
// Space Complexity: O(1) in Big-O terms (since 26² is constant), or O(26²) explicitly

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let (mut path, x, mut res) = (vec![vec![i64::MAX / 2; 26]; 26], 'a' as usize, 0);

        for i in 0..cost.len() {
            let a = original[i] as usize - x;
            let b = changed[i] as usize - x;

            path[a][b] = path[a][b].min(cost[i] as i64);
        }

        for i in 0..26 {
            path[i][i] = 0
        }
        for i in 0..26 {
            for a in 0..26 {
                for b in 0..26 {
                    path[a][b] = path[a][b].min(path[a][i] + path[i][b])
                }
            }
        }

        for (a, b) in source.chars().zip(target.chars()) {
            let (a, b) = (a as usize - x, b as usize - x);
            let p = path[a][b];

            if p == i64::MAX / 2 {
                return -1;
            }

            res += p
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost_1() {
        let (source, target, original, changed, cost) = (
            "abcd".to_string(),
            "acbe".to_string(),
            vec!['a', 'b', 'c', 'c', 'e', 'd'],
            vec!['b', 'c', 'b', 'e', 'b', 'e'],
            vec![2, 5, 5, 1, 2, 20],
        );
        let result = Solution::minimum_cost(source, target, original, changed, cost);

        assert_eq!(result, 28)
    }

    #[test]
    fn test_minimum_cost_2() {
        let (source, target, original, changed, cost) = (
            "aaaa".to_string(),
            "bbbb".to_string(),
            vec!['a', 'c'],
            vec!['c', 'b'],
            vec![1, 2],
        );
        let result = Solution::minimum_cost(source, target, original, changed, cost);

        assert_eq!(result, 12)
    }
}
