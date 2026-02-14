// LeetCode Problem 3721. Longest Balanced Subarray II
// Difficulty: Hard

// Time Complexity: O(n + mx) - where n is the length of nums and mx is the maximum
// value in nums, since we perform linear passes over the array and use frequency
// arrays sized by mx.

// Space Complexity: O(n + mx) - where n is used for the next array and mx for
// the count and last arrays that track frequencies and last occurrences.

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let (n, mx) = (nums.len(), *nums.iter().max().unwrap());
        let (mut ans, mut count, mut last, mut next, mut uniq) = (
            0_usize,
            vec![0; mx as usize + 1],
            vec![n; mx as usize + 1],
            vec![n; n],
            [0; 2],
        );

        for (i, &x) in nums.iter().enumerate() {
            Self::update(x, 1, &mut count, &mut uniq);

            if last[x as usize] < n {
                next[last[x as usize]] = i;
            }
            last[x as usize] = i;
        }

        let mut j = n;
        for i in 0..n - 1 {
            if n - i <= ans {
                break;
            }

            while j - i > ans {
                if uniq[0] == uniq[1] {
                    ans = j - i;
                    break;
                }

                j -= 1;

                Self::update(nums[j], -1, &mut count, &mut uniq);
            }

            while j < next[i] {
                Self::update(nums[j], 1, &mut count, &mut uniq);
                j += 1;
            }

            Self::update(nums[i], -1, &mut count, &mut uniq);
        }

        ans as i32
    }

    fn update(x: i32, val: i32, count: &mut Vec<i32>, uniq: &mut [i32; 2]) {
        let xu = x as usize;
        count[xu] += val;
        if count[xu] == 0 && val < 0 {
            uniq[xu & 1] -= 1;
        } else if count[xu] == val && val > 0 {
            uniq[xu & 1] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_balanced_1() {
        let input = vec![2, 5, 4, 3];
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 4)
    }

    #[test]
    fn test_longest_balanced_2() {
        let input = vec![3, 2, 2, 5, 4];
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 5)
    }

    #[test]
    fn test_longest_balanced_3() {
        let input = vec![1, 2, 3, 2];
        let result = Solution::longest_balanced(input);
        assert_eq!(result, 3)
    }
}
