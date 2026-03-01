// LeetCode Problem 1356. Sort Integers by The Number of 1 Bits
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the size of the vector arr
// Space Complexity: O(1) - constant space, no new data structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|a, b| a.count_ones().cmp(&b.count_ones()).then(a.cmp(b)));
        arr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_by_bits_1() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![0, 1, 2, 4, 8, 3, 5, 6, 7];
        let result = Solution::sort_by_bits(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sort_by_bits_2() {
        let input = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        let expected = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        let result = Solution::sort_by_bits(input);
        assert_eq!(result, expected);
    }
}
