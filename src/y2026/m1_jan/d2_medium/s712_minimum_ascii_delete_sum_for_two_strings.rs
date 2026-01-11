// LeetCode Problem 712. Minimum ASCII Delete Sum for Two Strings
// Difficulty: Medium

// Time Complexity: O(m * n)
// A 2D dynamic programming table of size (m + 1) × (n + 1) is filled using nested loops,
// where m = length of s1 and n = length of s2.

// Space Complexity: O(m * n)
// The DP table stores the maximum ASCII sum of the common subsequence for all prefixes
// of s1 and s2, requiring O(m * n) extra space.

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1_len = s1.len();
        let s2_len = s2.len();

        let mut dp: Vec<Vec<u32>> = vec![vec![0; s2_len + 1]; s1_len + 1];

        for i in 0..s1_len {
            for j in 0..s2_len {
                if s1.chars().nth(i) == s2.chars().nth(j) {
                    dp[i + 1][j + 1] = s1.chars().nth(i).unwrap() as u32 + dp[i][j];
                } else {
                    dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }

        let s1_ascii_sum: u32 = s1.bytes().map(|c| c as u32).sum();
        let s2_ascii_sum: u32 = s2.bytes().map(|c| c as u32).sum();

        let ans = s1_ascii_sum + s2_ascii_sum - 2 * dp[s1_len][s2_len];

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_delete_sum_1() {
        let result = Solution::minimum_delete_sum("sea".to_owned(), "eat".to_owned());
        assert_eq!(result, 231);
    }

    #[test]
    fn test_minimum_delete_sum_2() {
        let result = Solution::minimum_delete_sum("delete".to_owned(), "leet".to_owned());
        assert_eq!(result, 403);
    }
}
