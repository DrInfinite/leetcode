// LeetCode Problem 1390. Four Divisors
// Difficulty: Medium
//
// Time Complexity: O(n∗Sqrt(m)) - where n is the length of the array and m is a possible list of
// prime numbers
// Space Complexity: O(1) - constant size where n is the size of the constant

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for x in nums {
            let mut n = x;
            let mut p = 0;
            let mut q = 0;
            let mut count = 0;

            let mut i = 2;

            while i * i <= n && count <= 2 {
                if n % i == 0 {
                    count += 1;
                    match count == 1 {
                        true => p = i,
                        false => q = i,
                    }
                    while n % i == 0 {
                        n /= i;
                    }
                }
                i += 1;
            }

            if n > 1 {
                count += 1;
                match count == 1 {
                    true => p = n,
                    false => q = n,
                }
            }

            match count {
                2 if p * q == x => ans += 1 + p + q + x,
                1 if p * p * p == x => ans += 1 + p + p * p + x,
                _ => println!("Should never execute, if it does you f***ed up!!"),
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_four_divisors_1() {
        let nums = vec![21, 4, 7];
        assert_eq!(Solution::sum_four_divisors(nums), 32);
    }

    #[test]
    fn test_sum_four_divisors_2() {
        let nums = vec![21, 21];
        assert_eq!(Solution::sum_four_divisors(nums), 64);
    }

    #[test]
    fn test_sum_four_divisors_3() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::sum_four_divisors(nums), 0);
    }
}
