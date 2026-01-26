// LeetCode Problem 1984. Minimum Difference Between Highest and Lowest of K Scores
// Difficulty: Easy
//
// Time Complexity: o(n * log n) - where n * log n is the sorting time complexity
// Space Complexity: O(log n) - where n is the size of the vector nums

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.windows(k as usize)
            .map(|pair| pair[(k - 1) as usize] - pair[0])
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_difference_1() {
        let input = vec![90];
        let result = Solution::minimum_difference(input, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_minimum_difference_2() {
        let input = vec![9, 4, 1, 7];
        let result = Solution::minimum_difference(input, 2);
        assert_eq!(result, 2);
    }
}
