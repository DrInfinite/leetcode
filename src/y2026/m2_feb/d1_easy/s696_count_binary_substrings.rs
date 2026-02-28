// LeetCode Problem 696. Count Binary Substrings
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the size of the string s
// Space Complexity: O(1) - constant space, as only minor auxilary space is
// allocated for  count, prev and element c

#![allow(dead_code)]

use std::cmp::min;

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut chunks = s.as_bytes().chunk_by(|&a, &b| a == b).map(|c| c.len());

        let mut count = 0;
        let mut prev = chunks.next().expect("at least one chunk");

        for c in chunks {
            count += min(prev, c);
            prev = c;
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_binary_substrings_1() {
        let result = Solution::count_binary_substrings("00110011".to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn test_count_binary_substrings_2() {
        let result = Solution::count_binary_substrings("10101".to_string());
        assert_eq!(result, 4);
    }
}
