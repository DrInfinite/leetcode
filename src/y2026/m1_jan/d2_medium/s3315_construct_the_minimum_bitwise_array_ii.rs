// LeetCode Problem 3315. Construct the Minimum Bitwise Array II
// Difficulty: Medium

// Time Complexity: O(n) - where n is the size of the array nums
// Space Complexity: O(1) - constant space as no new memory is allocated

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.iter_mut().for_each(|num| match num.trailing_ones() {
            0 => *num = -1,
            x => *num ^= 1 << (x - 1),
        });
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_bitwise_array_1() {
        let input = vec![2, 3, 5, 7];
        let expected = vec![-1, 1, 4, 3];
        let result = Solution::min_bitwise_array(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_min_bitwise_array_2() {
        let input = vec![11, 13, 31];
        let expected = vec![9, 12, 15];
        let result = Solution::min_bitwise_array(input);
        assert_eq!(result, expected);
    }
}
