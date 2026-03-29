// LeetCode Problem 2839. Check if Strings Can be Made Equal With Operations I
// Difficulty: Easy
//
// Time Complexity: O(1) - constant time, string size is always 4
// Space Complexity: O(1) - constant space, fixed space allocated in all cases

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn can_be_equal(a: String, b: String) -> bool {
        if a == b {
            return true;
        }

        let s0 = b.get(0..1).unwrap();
        let s1 = b.get(1..2).unwrap();
        let s2 = b.get(2..3).unwrap();
        let s3 = b.get(3..4).unwrap();

        if a == format!("{s2}{s1}{s0}{s3}") {
            return true;
        }

        if a == format!("{s0}{s3}{s2}{s1}") {
            return true;
        }

        if a == format!("{s2}{s3}{s0}{s1}") {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_ones_segment_1() {
        let result = Solution::can_be_equal("abcd".to_string(), "cdab".to_string());
        assert!(result);
    }

    #[test]
    fn test_check_ones_segment_2() {
        let result = Solution::can_be_equal("abcd".to_string(), "dcab".to_string());
        assert!(!result);
    }
}
