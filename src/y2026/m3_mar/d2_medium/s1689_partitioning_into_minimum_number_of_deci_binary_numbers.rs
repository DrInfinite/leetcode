// LeetCode Problem 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of the string
// Space Complexity: O(1) - constant space, no new data-structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut res = 0;

        for s in n.chars() {
            if s.to_digit(10).unwrap() > res {
                res = s.to_digit(10).unwrap();
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_partitions_1() {
        let result = Solution::min_partitions("32".to_string());
        assert_eq!(result, 3)
    }

    #[test]
    fn test_min_partitions_2() {
        let result = Solution::min_partitions("82734".to_string());
        assert_eq!(result, 8)
    }

    #[test]
    fn test_min_partitions_3() {
        let result = Solution::min_partitions("27346209830709182346".to_string());
        assert_eq!(result, 9)
    }
}
