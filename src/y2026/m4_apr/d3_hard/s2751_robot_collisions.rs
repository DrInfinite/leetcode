// LeetCode Problem 2751. Robot Collisions
// Difficulty: Hard

// Time Complexity: O(m log m) - where m is the size of the positions array
// Space Complexity: O(n) - where n is the size of the positions array

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let mut p = (0..n)
            .map(|i| (positions[i], i))
            .collect::<Vec<(i32, usize)>>();
        p.sort();

        let directions = directions.chars().collect::<Vec<char>>();
        let (mut temp, mut sk) = (vec![], vec![]);

        for i in 0..n {
            let k = p[i].1;

            if directions[k] == 'R' {
                sk.push((k, healths[k]));
                continue;
            }

            let mut h = healths[k];

            while !sk.is_empty() {
                let sz = sk.len();
                if sk[sz - 1].1 >= h {
                    break;
                }
                h -= 1;
                sk.pop();
            }

            if !sk.is_empty() {
                let sz = sk.len();
                if sk[sz - 1].1 == h {
                    sk.pop();
                    continue;
                }
                sk[sz - 1].1 -= 1;
                continue;
            }
            temp.push((k, h));
        }
        for (i, h) in sk {
            temp.push((i, h));
        }
        temp.sort();

        temp.into_iter().map(|a| a.1).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_survived_robots_healths_1() {
        let result = Solution::survived_robots_healths(
            [5, 4, 3, 2, 1].to_vec(),
            [2, 17, 9, 15, 10].to_vec(),
            "RRRRR".to_string(),
        );

        assert_eq!(result, [2, 17, 9, 15, 10].to_vec())
    }

    #[test]
    fn test_survived_robots_healths_2() {
        let result = Solution::survived_robots_healths(
            [3, 5, 2, 6].to_vec(),
            [10, 10, 15, 12].to_vec(),
            "RLRL".to_string(),
        );

        assert_eq!(result, [14].to_vec())
    }

    #[test]
    fn test_survived_robots_healths_3() {
        let result = Solution::survived_robots_healths(
            [1, 2, 5, 6].to_vec(),
            [10, 10, 11, 11].to_vec(),
            "RLRL".to_string(),
        );

        assert_eq!(result, [].to_vec())
    }
}
