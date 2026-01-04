// LeetCode Problem 961. N-Repeated Element in Size 2N Array
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the array
// Space Complexity: O(1) - constant space where n is the size of the constant

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut same_sides = 6_i64;
        let mut unique_cells = 6_i64;

        (1..n).for_each(|_| {
            unique_cells = ((same_sides + unique_cells) * 2) % 1000000007;
            same_sides += unique_cells;
        });

        ((same_sides + unique_cells) % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_ways_1() {
        assert_eq!(Solution::num_of_ways(1), 12);
    }

    #[test]
    fn test_num_of_ways_2() {
        assert_eq!(Solution::num_of_ways(5000), 30228214);
    }
}
