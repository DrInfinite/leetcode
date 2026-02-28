// LeetCode Problem 761. Special Binary String
// Difficulty: Hard

// Time Complexity: O(n ^ 2) - where n is the length of string s
// Space Complexity: O(n) - where n is the length of string s

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        Self::make_largest(&s)
    }

    fn make_largest(s: &str) -> String {
        let mut count = 0;
        let mut start = 0;
        let mut substrings = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '1' {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                let substring = &s[start..=i];
                if substring.len() > 2 {
                    let inner = &substring[1..substring.len() - 1];
                    let processed = Self::make_largest(inner);
                    substrings.push(format!("1{}0", processed));
                } else {
                    substrings.push(substring.to_string());
                }
                start = i + 1;
            }
        }

        substrings.sort_by(|a, b| b.cmp(a));
        substrings.join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_largest_special_1() {
        let result = Solution::make_largest_special("11011000".to_string());
        assert_eq!(result, "11100100".to_string())
    }

    #[test]
    fn test_make_largest_special_2() {
        let result = Solution::make_largest_special("10".to_string());
        assert_eq!(result, "10".to_string())
    }
}
