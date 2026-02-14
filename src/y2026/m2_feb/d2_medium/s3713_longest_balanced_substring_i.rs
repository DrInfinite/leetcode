// LeetCode Problem 3713. Longest Balanced Substring I
// Difficulty: Medium

// Time Complexity: O(n^2) - where n is the length of the string, since we check
// all O(n^2) substrings and each balance check runs in constant time over a
// fixed 26-character alphabet.

// Space Complexity: O(1) - since the hash array has a fixed size of 26
// lowercase letters regardless of input length.

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        fn is_balanced(hash: &Vec<i32>) -> bool {
            let mut first = 0;

            for item in hash {
                if first == 0 && *item > 0 {
                    first = *item;
                }

                if *item != 0 && *item != first {
                    return false;
                }
            }
            true
        }

        let s: Vec<u8> = s.as_bytes().to_vec();
        let mut result = 0;

        for idx in 0..s.len() {
            let mut hash = vec![0; 26];

            for idx2 in idx..s.len() {
                hash[(s[idx2] - 'a' as u8) as usize] += 1;

                if is_balanced(&hash) {
                    result = result.max(idx2 - idx + 1);
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_balanced_1() {
        let input = "abbac".to_string();
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 4)
    }
    #[test]
    fn test_longest_balanced_2() {
        let input = "zzabccy".to_string();
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 4)
    }
    #[test]
    fn test_longest_balanced_3() {
        let input = "aba".to_string();
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 2)
    }
}
