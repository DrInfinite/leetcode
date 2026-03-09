// LeetCode Problem 401. Binary Watch
// Difficulty: Easy
//
// Time Complexity: O(720) - where 720 is the number of possible time combinations
// Space Complexity: O(720) - where 720 is the number of possible time combinations

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        (0..12u8)
            .flat_map(|h| (0..60u8).map(move |m| (h, m)))
            .filter(|(h, m)| h.count_ones() + m.count_ones() == turned_on as u32)
            .map(|(h, m)| format!("{h}:{m:02}"))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::utils::strings::Strings;

    use super::*;

    #[test]
    fn test_read_binary_watch_1() {
        let result = Solution::read_binary_watch(1);
        let expected = Strings::vector_string(vec![
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_read_binary_watch_2() {
        let result = Solution::read_binary_watch(9);
        let expected = Strings::vector_string(vec![]);
        assert_eq!(result, expected)
    }
}
