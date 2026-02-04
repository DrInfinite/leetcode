// LeetCode Problem 3640. Trionic Array II
// Difficulty: Hard

// Time Complexity: O(n) - n is the length of the input array; each pointer
// (i, p, q, k) only moves forward (or backward once for left extension),
// so every element is processed a constant number of times overall.

// Space Complexity: O(1) - Only a constant amount of extra space is used for
// pointers and running sums, independent of n.

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans = i64::MIN;
        let mut i = 0;

        while i < n {
            // Find peak p (end of increasing segment)
            let mut p = i;
            while p + 1 < n && nums[p] < nums[p + 1] {
                p += 1;
            }

            // Need at least 2 elements in first segment
            if p == i {
                i += 1;
                continue;
            }

            // Find valley q (end of decreasing segment)
            let mut q = p;
            while q + 1 < n && nums[q] > nums[q + 1] {
                q += 1;
            }

            // Need valid decreasing segment and room for third segment
            if q == p || q + 1 >= n || nums[q + 1] <= nums[q] {
                i = q.max(1);
                continue;
            }

            // Core trionic: [p-1, p, ..., q, q+1]
            let mut sum = nums[p - 1] as i64 + nums[p] as i64;
            for k in (p + 1)..=q + 1 {
                sum += nums[k] as i64;
            }

            // Extend right: find max increasing suffix starting at q+2
            let mut right_max = 0i64;
            let mut right_sum = 0i64;
            let mut k = q + 2;
            while k < n && nums[k] > nums[k - 1] {
                right_sum += nums[k] as i64;
                right_max = right_max.max(right_sum);
                k += 1;
            }

            // Extend left: find max increasing prefix ending at p-2
            let mut left_max = 0i64;
            let mut left_sum = 0i64;
            let mut k = p as i32 - 2;
            while k >= i as i32 {
                left_sum += nums[k as usize] as i64;
                left_max = left_max.max(left_sum);
                k -= 1;
            }

            ans = ans.max(sum + left_max + right_max);
            i = q;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_trionic_1() {
        let input = vec![0, -2, -1, -3, 0, 2, -1];
        let result = Solution::max_sum_trionic(input);
        assert_eq!(result, -4);
    }

    #[test]
    fn test_max_sum_trionic_2() {
        let input = vec![1, 4, 2, 7];
        let result = Solution::max_sum_trionic(input);
        assert_eq!(result, 14);
    }
}
