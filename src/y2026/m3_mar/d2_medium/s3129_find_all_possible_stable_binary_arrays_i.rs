// LeetCode Problem 3129. Find All Possible Stable Binary Arrays I
// Difficulty: Medium
//
// Time Complexity: O(m * n * limit) - where m and n are the counts of zeroes and ones
// respectively, and limit is the amount of work done for each state
// Space Complexity: O(m * n) - where m and n are the counts of zeroes and ones
// respectively

#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    fn f(cache: &mut HashMap<(i32, i32), i32>, a: i32, b: i32, limit: i32, t: i32) -> i32 {
        if a == 0 {
            return 0;
        }

        if b == 0 {
            return if a <= limit { return 1 } else { 0 };
        }

        if !cache.contains_key(&(a, b)) {
            let z = (0..a.min(limit))
                .map(|d| Self::f(cache, b, a - 1 - d, limit, 0))
                .fold(t, |acc, x| (acc + x).rem_euclid(1_000_000_007));
            cache.insert((a, b), z);
        }

        cache[&(a, b)]
    }

    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let mut cache = HashMap::new();
        (Self::f(&mut cache, zero, one, limit, 0) + Self::f(&mut cache, one, zero, limit, 0))
            .rem_euclid(1_000_000_007)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_stable_arrays_1() {
        let result = Solution::number_of_stable_arrays(1, 1, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_number_of_stable_arrays_2() {
        let result = Solution::number_of_stable_arrays(1, 2, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_number_of_stable_arrays_3() {
        let result = Solution::number_of_stable_arrays(3, 3, 2);
        assert_eq!(result, 14);
    }
}
