// LeetCode Problem 744. Find Smallest Letter Greater Than Target
// Difficulty: Easy

// Time Complexity: O(n)  // In the worst case, the jump-based search can traverse
// across the array elements.
// Space Complexity: O(1) // Uses a constant amount of extra space regardless of input size.

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let n = letters.len();
        let mut jump = n / 2;
        let mut curr = jump;

        while curr > 0 && curr < n {
            jump /= 2;

            if jump == 0 {
                jump = 1;
            }

            if letters[curr] <= target {
                curr += jump;
            } else if letters[curr - 1] > target {
                curr -= jump;
            } else {
                return letters[curr];
            }
        }

        letters[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greatest_letter_1() {
        let input = vec!['c', 'f', 's'];
        let result = Solution::next_greatest_letter(input, 'a');
        assert_eq!(result, 'c');
    }

    #[test]
    fn test_next_greatest_letter_2() {
        let input = vec!['c', 'f', 'j'];
        let result = Solution::next_greatest_letter(input, 'c');
        assert_eq!(result, 'f');
    }

    #[test]
    fn test_next_greatest_letter_3() {
        let input = vec!['x', 'x', 'y', 'y'];
        let result = Solution::next_greatest_letter(input, 'z');
        assert_eq!(result, 'x');
    }
}
