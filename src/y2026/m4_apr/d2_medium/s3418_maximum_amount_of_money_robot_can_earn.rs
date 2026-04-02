// LeetCode Problem 3418. Maximum Amount of Money Robot Can Earn
// Difficulty: Medium

// Time Complexity: O(r * c) - where r is the number of rows and c is the number
// of cols in the grid coins

// Space Complexity: O(c) - where c is the number of cols in the grid coins

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let mut line = vec![(i32::MIN, i32::MIN, i32::MIN); coins[0].len()];
        let mut left = (0, 0, 0);
        line[0] = left;

        for crow in coins {
            left = (i32::MIN, i32::MIN, i32::MIN);

            for (up, &coins) in line.iter_mut().zip(crow.iter()) {
                left.2 = (coins + left.2.max(up.2)).max(left.1).max(up.1);
                left.1 = (coins + left.1.max(up.1)).max(left.0).max(up.0);
                left.0 = coins + left.0.max(up.0);

                *up = left;
            }
        }

        left.2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::arr_vec::ToNestedVec;

    #[test]
    fn test_maximum_amount_1() {
        let input = [[0, 1, -1], [1, -2, 3], [2, -3, 4]].to_nested_vec();
        let result = Solution::maximum_amount(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_maximum_amount_2() {
        let input = [[10, 10, 10], [10, 10, 10]].to_nested_vec();
        let result = Solution::maximum_amount(input);

        assert_eq!(result, 40);
    }
}
