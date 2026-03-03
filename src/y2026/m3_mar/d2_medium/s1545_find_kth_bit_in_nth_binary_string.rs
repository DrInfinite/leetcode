// LeetCode Problem 1545. Find Kth Bit in Nth Binary String
// Difficulty: Medium
//
// Time Complexity: O(n) - where n is the size of the matrix grid
// Space Complexity: O(1) - constant space, no new data-structures created

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }

        let len = (1 << n) - 1;

        match k.cmp(&((len >> 1) + 1)) {
            std::cmp::Ordering::Less => Self::find_kth_bit(n - 1, k),
            std::cmp::Ordering::Equal => '1',
            std::cmp::Ordering::Greater => {
                char::from(Self::find_kth_bit(n - 1, len - k + 1) as u8 ^ 1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_kth_bit_1() {
        let result = Solution::find_kth_bit(3, 1);
        assert_eq!(result, '0');
    }

    #[test]
    fn test_find_kth_bit_2() {
        let result = Solution::find_kth_bit(4, 11);
        assert_eq!(result, '1');
    }
}
