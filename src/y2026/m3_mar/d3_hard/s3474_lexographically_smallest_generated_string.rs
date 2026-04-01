// LeetCode Problem 3474. Lexicographically Smallest Generated String
// Difficulty: Hard

// Time Complexity: O(m * n) - where m and n are the lengths of str1 and str2
// respectively
// Space Complexity: O(m + n) - where m and n are the lengths of str1 and str2
// respectively

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let len1 = str1.len();
        let len2 = str2.len();
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let mut res = vec![b'A'; len1 + len2 - 1];

        let mut insert = usize::MAX;
        for i in (0..len1).rev() {
            if s1[i] == b'T' {
                if insert - i < len2 && s2[insert - i..len2] != s2[0..len2 - insert + i] {
                    return String::new();
                }

                res[i..i + len2].copy_from_slice(s2);
                insert = i
            }
        }

        'outer: for i in 0..len1 {
            if s1[i] == b'F' {
                let mut freeidx = usize::MAX;

                for j in (0..len2).rev() {
                    if res[i + j] | 32 != s2[j] {
                        continue 'outer;
                    }
                    if freeidx == usize::MAX && res[i + j] & 32 == 0 {
                        freeidx = i + j;
                    }
                }
                if freeidx == usize::MAX {
                    return String::new();
                }
                res[freeidx] = b'b';
            }
        }

        res.iter().map(|x| (32 | *x) as char).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_string_1() {
        let result = Solution::generate_string("TFTF".to_string(), "ab".to_string());
        assert_eq!(result, "ababa".to_string());
    }

    #[test]
    fn test_generate_string_2() {
        let result = Solution::generate_string("TFTF".to_string(), "abc".to_string());
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_generate_string_3() {
        let result = Solution::generate_string("F".to_string(), "d".to_string());
        assert_eq!(result, "a".to_string());
    }
}
