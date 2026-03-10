// LeetCode Problem 3013. Divide an Array Into Subarrays With Minimum Cost II
// Difficulty: Hard

// Time Complexity: O(m * n * limit) - where m and n are the counts of zeroes and ones
// respectively, and limit is the amount of work done for each state
// Space Complexity: O(m * n) - where m and n are the counts of zeroes and ones
// respectively

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let (z, o, l) = (zero as usize, one as usize, limit as usize);
        let modv = 1_000_000_007;

        let mut dp = vec![vec![[0i64; 2]; o + 1]; z + 1];

        (1..=z.min(l)).for_each(|i| {
            dp[i][0][0] = 1;
        });

        (1..=o.min(l)).for_each(|j| {
            dp[0][j][1] = 1;
        });

        for i in 1..=z {
            for j in 1..=o {
                let over0 = if i > l { dp[i - l - 1][j][1] } else { 0 };

                let over1 = if j > l { dp[i][j - l - 1][0] } else { 0 };

                dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1] - over0 + modv) % modv;

                dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1] - over1 + modv) % modv;
            }
        }

        ((dp[z][o][0] + dp[z][o][1]) % modv) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_stable_arrays_1() {
        let result = Solution::number_of_stable_arrays(1, 1, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_number_of_stable_arrays_2() {
        let result = Solution::number_of_stable_arrays(1, 2, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_number_of_stable_arrays_3() {
        let result = Solution::number_of_stable_arrays(3, 3, 2);
        assert_eq!(result, 14);
    }
}
