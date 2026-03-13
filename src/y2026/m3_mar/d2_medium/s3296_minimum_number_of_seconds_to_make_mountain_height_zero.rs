// LeetCode Problem 3296. Minimum Number of Seconds to Make Mountain Height Zero
// Difficulty: Medium
//
// Time Complexity: O(h log n) - where h is the size of the binary heap q and
// n is the mountain height
// Space Complexity: O(n) - where n is the mountain height

#![allow(dead_code)]
struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut q = worker_times
            .into_iter()
            .map(|t| (Reverse(t as i64), t as i64, 1))
            .collect::<BinaryHeap<(Reverse<i64>, i64, i64)>>();

        let mut time = 0;

        (0..mountain_height).for_each(|_| {
            let (Reverse(tt), t, f) = q.pop().unwrap();
            time = tt;
            q.push((Reverse(tt + t * (f + 1)), t, f + 1));
        });

        time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_number_of_seconds_1() {
        let result = Solution::min_number_of_seconds(4, vec![2, 1, 1]);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_min_number_of_seconds_2() {
        let result = Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]);
        assert_eq!(result, 12)
    }

    #[test]
    fn test_min_number_of_seconds_3() {
        let result = Solution::min_number_of_seconds(5, vec![1]);
        assert_eq!(result, 15)
    }
}
