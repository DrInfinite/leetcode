// LeetCode Problem 67. Add Binary
// Difficulty: Easy
//
// Time Complexity: O(max(m, n)) - where m and n are the lengths of strings
// a and b respectively
// Space Complexity: O(max(m, n)) - where m and n are the lengths of strings
// a and b respectively

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        use std::iter;

        let mut carry = 0;
        let mut cur_sum = 0;

        let mut char_vec = a
            .as_bytes()
            .iter()
            .rev()
            .chain(iter::repeat(&b'0'))
            .zip(b.as_bytes().iter().rev().chain(iter::repeat(&b'0')))
            .take(a.len().max(b.len()))
            .map(|(ac, bc)| {
                cur_sum = (*ac - b'0') + (*bc - b'0') + carry;
                carry = cur_sum / 2;
                match cur_sum % 2 {
                    1 => '1',
                    _ => '0',
                }
            })
            .collect::<Vec<_>>();

        if carry == 1 {
            char_vec.push('1');
        }

        char_vec.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary_1() {
        let (a, b) = ("11".to_string(), "1".to_string());
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "100".to_string())
    }

    #[test]
    fn test_add_binary_2() {
        let (a, b) = ("1010".to_string(), "1011".to_string());
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "10101".to_string())
    }
}
