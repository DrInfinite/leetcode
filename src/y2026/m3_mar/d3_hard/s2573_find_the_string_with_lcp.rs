// LeetCode Problem 2573. Find the String with LCP
// Difficulty: Medium

// Time Complexity: O(n^2) - where n is the size of the grid p
// Space Complexity: O(n) - where n is the size of the grid p

#![allow(dead_code)]
struct Solution;

impl Solution {
    // 7ms
    pub fn find_the_string(p: Vec<Vec<i32>>) -> String {
        let n = p.len();
        let (mut s, mut c) = (vec![0; n], 96);
        for i in 0..n {
            if s[i] == 0 {
                c += 1;
                if c > 122 {
                    return "".into();
                }
                for j in 0..n {
                    if p[i][j] > 0 {
                        s[j] = c
                    }
                }
            }
        }
        if (0..n).all(|i| {
            (0..n).all(|j| {
                p[i][j]
                    == (s[i] == s[j]) as i32
                        * (1 + if i.max(j) < n - 1 { p[i + 1][j + 1] } else { 0 })
            })
        }) {
            String::from_utf8(s).unwrap()
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_find_the_string_1() {
        let input = [[4, 0, 2, 0], [0, 3, 0, 1], [2, 0, 2, 0], [0, 1, 0, 1]].to_nested_vec();
        let result = Solution::find_the_string(input);

        assert_eq!(result, "abab".to_string())
    }

    #[test]
    fn test_find_the_string_2() {
        let input = [[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 1]].to_nested_vec();
        let result = Solution::find_the_string(input);

        assert_eq!(result, "aaaa".to_string())
    }

    #[test]
    fn test_find_the_string_3() {
        let input = [[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 3]].to_nested_vec();
        let result = Solution::find_the_string(input);

        assert_eq!(result, "".to_string())
    }
}
