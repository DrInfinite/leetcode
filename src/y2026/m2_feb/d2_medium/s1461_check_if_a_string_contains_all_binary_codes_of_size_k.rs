// LeetCode Problem 1461. Check If a String Contains All Binary Codes of Size K
// Difficulty: Medium
//
// Time Complexity: O(n * k) - where n is the length of the string, k is size of window
// Space Complexity: O(1) - constant space, no extra space allocated

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        s.as_bytes()
            .windows(k as usize)
            .collect::<std::collections::HashSet<_>>()
            .len()
            == 1 << k
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_all_codes_1() {
        let result = Solution::has_all_codes("00110110".to_string(), 2);
        assert!(result)
    }

    #[test]
    fn test_has_all_codes_2() {
        let result = Solution::has_all_codes("0110".to_string(), 1);
        assert!(result)
    }

    #[test]
    fn test_has_all_codes_3() {
        let result = Solution::has_all_codes("0110".to_string(), 2);
        assert!(!result)
    }
}
