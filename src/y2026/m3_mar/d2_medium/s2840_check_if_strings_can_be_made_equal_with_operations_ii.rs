// LeetCode Problem 2840. Check if Strings Can be Made Equal With Operations II
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of strings s1 and s2
// Space Complexity: O(1) - constant space, fixed space allocated in all cases

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        let mut diff = [[0i32; 2]; 26];

        for (i, (&c1, &c2)) in s1.iter().zip(s2.iter()).enumerate() {
            let p = i % 2;
            diff[(c1 - b'a') as usize][p] += 1;
            diff[(c2 - b'a') as usize][p] -= 1;
        }

        diff.iter().all(|row| row[0] == 0 && row[1] == 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_ones_segment_1() {
        let result = Solution::check_strings("abcdba".to_string(), "cabdab".to_string());
        assert!(result);
    }

    #[test]
    fn test_check_ones_segment_2() {
        let result = Solution::check_strings("abe".to_string(), "bea".to_string());
        assert!(!result);
    }
}
