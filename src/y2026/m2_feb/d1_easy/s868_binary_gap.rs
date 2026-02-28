// LeetCode Problem 868. Binary Gap
// Difficulty: Easy
//
// Time Complexity: O(log n)
// The number is right-shifted until it becomes 0. Since each shift removes one bit,
// the loop runs once per bit in n. The number of bits in n is proportional to log₂(n).
//
// Space Complexity: O(1)
// The solution uses a constant amount of extra space. The iterator chain does not
// allocate additional memory proportional to input size, and only a few variables
// are maintained throughout execution.

#![allow(dead_code)]

use std::cmp::Ord;
use std::iter::successors;
use std::mem::replace;

struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        successors(Some(n), |n| Some(n >> 1))
            .take_while(|n| n.is_positive())
            .enumerate()
            .filter_map(|(i, n)| (n & 1 == 1).then_some(i))
            .scan(usize::MAX, |p, i| {
                Some(i.saturating_sub(replace(p, i)) as i32)
            })
            .fold(0, Ord::max)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_gap_1() {
        let result = Solution::binary_gap(22);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_binary_gap_2() {
        let result = Solution::binary_gap(8);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_binary_gap_3() {
        let result = Solution::binary_gap(5);
        assert_eq!(result, 2);
    }
}
