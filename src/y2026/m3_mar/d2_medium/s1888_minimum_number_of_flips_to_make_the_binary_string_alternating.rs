// LeetCode Problem 1888. Minimum Number of Flips to Make the Binary String Alternating
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of the string
// Space Complexity: O(n) - where n is the size of the string

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let doubled: Vec<char> = s.chars().chain(s.chars()).collect();

        let mut cost1: i32 = 0;

        (0..n).for_each(|i| {
            if doubled[i] != (if i % 2 == 0 { '0' } else { '1' }) {
                cost1 += 1;
            }
        });

        let mut result = cost1.min(n as i32 - cost1);
        let pat2_n = if n.is_multiple_of(2) { '1' } else { '0' };

        (0..n).for_each(|r| {
            let outgoing_is_one = doubled[r] == '1';
            let incoming_mismatch = doubled[r + n] != pat2_n;

            cost1 = (n as i32 - cost1) - (if outgoing_is_one { 0 } else { 1 })
                + (if incoming_mismatch { 1 } else { 0 });

            result = result.min(cost1).min(n as i32 - cost1);
        });

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_flips_1() {
        let result = Solution::min_flips("111000".to_string());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_min_flips_2() {
        let result = Solution::min_flips("010".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_min_flips_3() {
        let result = Solution::min_flips("1110".to_string());
        assert_eq!(result, 1);
    }
}
