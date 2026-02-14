// LeetCode Problem 3714. Longest Balanced Substring II
// Difficulty: Medium

// Time Complexity: O(n) — where n is the length of the string s

// Space Complexity: O(n) - where n is the max number of elements that can be stored
// in the hash.

#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let (mut ans, mut hash) = (0, HashMap::<[i32; 3], usize>::new());

        for k in 0..7 {
            hash.insert([0; 3], 0);
            let mut arr = [0; 3];

            for (i, b) in s.bytes().enumerate() {
                arr[(b - b'a') as usize] += 1;

                let mut lo = i32::MAX;

                for d in 0..3 {
                    if (k & (1 << d)) == 0 {
                        lo = lo.min(arr[d]);
                    }
                }

                for d in 0..3 {
                    if (k & (1 << d)) == 0 {
                        arr[d] -= lo;
                    }
                }

                if let Some(&j) = hash.get(&arr) {
                    ans = ans.max(i + 1 - j);
                } else {
                    hash.insert(arr.clone(), i + 1);
                }
            }

            hash.clear()
        }

        ans as i32
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
        let input = "aabcc".to_string();
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 3)
    }
    #[test]
    fn test_longest_balanced_3() {
        let input = "aba".to_string();
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 2)
    }
}
