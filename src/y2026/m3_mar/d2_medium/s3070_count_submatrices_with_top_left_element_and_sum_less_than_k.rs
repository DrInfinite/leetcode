// LeetCode Problem 3070. Count Submatrices with Top-Left Element and Sum Less Than k
// Difficulty: Medium
//
// Time Complexity: O(m * n) - where m is the number of rows and n is the number
// of cols in the grid respectively
// Space Complexity: O(1) - constant space, no new data structures generated

#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let leny = grid.len();
        let lenx = grid[0].len();

        grid.iter_mut().for_each(|row| {
            (1..lenx).for_each(|i| {
                row[i] += row[i - 1];
            })
        });

        let mut res = grid[0].iter().filter(|x| **x <= k).count();

        (1..leny).for_each(|j| {
            let (top, bottom) = grid.split_at_mut(j);
            let prevr = top.last().unwrap();
            let row = &mut bottom[0];

            for (curr, prev) in row.iter_mut().zip(prevr.iter()) {
                *curr += prev;
                if *curr <= k { res += 1 } else { break }
            }
        });

        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_submatrices_1() {
        let input = vec![vec![7, 6, 3], vec![6, 6, 1]];
        let result = Solution::count_submatrices(input, 18);

        assert_eq!(result, 4)
    }

    #[test]
    fn test_count_submatrices_2() {
        let input = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
        let result = Solution::count_submatrices(input, 20);

        assert_eq!(result, 6)
    }
}
