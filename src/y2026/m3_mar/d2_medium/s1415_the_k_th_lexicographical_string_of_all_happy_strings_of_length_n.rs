// LeetCode Problem 1415. The k-th Lexicographical String of All Happy Strings of Length n
// Difficulty: Medium
//
// Time Complexity: O(n * k) - where n is the length of the string, k is the kth
// happy string to be identified
// Space Complexity: O(n) - where n is the length of the string

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let mut s = String::with_capacity(n as usize);
        Self::back_tracking(n as usize, &mut k, &mut s);

        return s;
    }

    fn back_tracking(n: usize, k: &mut i32, s: &mut String) {
        if s.len() == n {
            *k -= 1;
            return;
        }

        for c in 'a'..='c' {
            if s.ends_with(c) {
                continue;
            }

            s.push(c);
            Self::back_tracking(n, k, s);

            if *k == 0 {
                break;
            }

            s.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_happy_string_1() {
        let result = Solution::get_happy_string(1, 3);
        assert_eq!(result, "c".to_string())
    }

    #[test]
    fn test_get_happy_string_2() {
        let result = Solution::get_happy_string(1, 4);
        assert_eq!(result, "".to_string())
    }

    #[test]
    fn test_get_happy_string_3() {
        let result = Solution::get_happy_string(3, 9);
        assert_eq!(result, "cab".to_string())
    }
}