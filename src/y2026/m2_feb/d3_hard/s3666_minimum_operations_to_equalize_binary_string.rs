// LeetCode Problem 3640. Trionic Array II
// Difficulty: Hard
//
// Time Complexity: O(n log n) - where n is the size of the string s
// Space Complexity: O(n) - where n is the size of the string

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let z = s.bytes().filter(|&c| c == b'0').count() as i32;
        let n = s.len() as i32;
        let mut st = vec![z];
        use std::collections::*;
        let mut avail: [BTreeSet<i32>; 2] =
            [(0..=n).step_by(2).collect(), (1..=n).step_by(2).collect()];
        avail[z as usize % 2].remove(&z);
        let mut d = 0;
        while !st.is_empty() {
            for z in std::mem::take(&mut st) {
                if z == 0 {
                    return d;
                }
                let zd = [(k + z - n).max(0), z.min(k)];
                let [l, r] = [z + k - 2 * zd[1], z + k - 2 * zd[0]];
                let avail = &mut avail[l as usize % 2];
                let s = st.len();
                st.extend(avail.range(l..=r).copied());
                for z in &st[s..] {
                    avail.remove(z);
                }
            }
            d += 1;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_operations_1() {
        let result = Solution::min_operations("110".to_string(), 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_min_operations_2() {
        let result = Solution::min_operations("0101".to_string(), 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_min_operations_3() {
        let result = Solution::min_operations("101".to_string(), 2);
        assert_eq!(result, -1);
    }
}
