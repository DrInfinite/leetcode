// LeetCode Problem 762. Prime Number of Set Bits in Binary Representation
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the number of elements between left and right
// Space Complexity: O(1) - constant space

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right).map(i32::count_ones).fold(0, |acc, n| {
            acc + i32::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31].contains(&n))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_prime_set_bits_1() {
        let result = Solution::count_prime_set_bits(6, 10);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_prime_set_bits_2() {
        let result = Solution::count_prime_set_bits(10, 15);
        assert_eq!(result, 5);
    }
}
