// LeetCode Problem 3013. Divide an Array Into Subarrays With Minimum Cost II
// Difficulty: Hard

// Time Complexity: O(n log n) - Each of the n elements is inserted/removed from
// a BTreeSet with O(log n) cost.

// Space Complexity: O(n) - Two BTreeSets together can store up to O(n) elements
// in the worst case.

#![allow(dead_code)]

use std::collections::BTreeSet;
struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let (mut lower, mut upper, mut s, mut result) = (
            BTreeSet::<(i32, i32)>::new(),
            BTreeSet::<(i32, i32)>::new(),
            nums[0] as i64,
            i64::MAX,
        );

        (1..).zip(&nums[1..]).for_each(|(j, &n)| {
            let i = j - dist - 1;

            if i >= 1 {
                let prev = nums[i as usize];

                if !upper.remove(&(prev, i)) {
                    s -= prev as i64;
                    lower.remove(&(prev, i));

                    let mm = upper.pop_first().unwrap();

                    s += mm.0 as i64;
                    lower.insert(mm);
                }
            }

            lower.insert((n, j));
            s += n as i64;

            if lower.len() as i32 + 1 == k {
                result = result.min(s);
                let mm = lower.pop_last().unwrap();
                s -= mm.0 as i64;
                upper.insert(mm);
            }
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost_1() {
        let input = vec![1, 3, 2, 6, 4, 2];
        let result = Solution::minimum_cost(input, 3, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_minimum_cost_2() {
        let input = vec![10, 1, 2, 2, 2, 1];
        let result = Solution::minimum_cost(input, 4, 3);
        assert_eq!(result, 15);
    }
}
