// LeetCode Problem 3634. Minimum Removals to Balance Array
// Difficulty: Medium
//
// Time Complexity: O(n^2) - where n is the size of the vector nums
// Space Complexity: O(1) - no addition memory is allocated, vector is directly modified

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let (k, mut i, mut ml) = (i64::from(k), 0, 0);

        for (j, r) in nums.iter().map(|&r| i64::from(r)).enumerate() {
            while r > i64::from(nums[i]) * k {
                i += 1;
            }
            ml = ml.max(j - i);
        }

        (nums.len() - ml - 1) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_removal_1() {
        let input = vec![2, 1, 5];
        let result = Solution::min_removal(input, 2);
        assert_eq!(result, 1)
    }

    #[test]
    fn test_min_removal_2() {
        let input = vec![1, 6, 2, 9];
        let result = Solution::min_removal(input, 3);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_min_removal_3() {
        let input = vec![4, 6];
        let result = Solution::min_removal(input, 2);
        assert_eq!(result, 0)
    }
}
