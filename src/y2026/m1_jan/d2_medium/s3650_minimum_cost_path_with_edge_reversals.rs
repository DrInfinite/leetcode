// LeetCode Problem 3650. Minimum Cost Path with Edge Reversals
// Difficulty: Medium

// Time Complexity: O(m log n) - where m is the size of the queue, and n is the
// size of the weighted graph

// Space Complexity: O(m + n) - where n is the number of nodes in the weighted
// graph and visited_array, and m is the maximum size of the BinaryHeap

#![allow(dead_code)]

use std::collections::BinaryHeap;
struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let (n_i64, mut graph, mut queue, mut visited) = (
            n as i64,
            vec![vec![]; n as usize],
            BinaryHeap::from([0]),
            vec![0; n as usize],
        );

        for x in edges {
            let (u, k, w) = (x[0] as usize, x[1] as usize, x[2] as i64);
            graph[u].push(w * n_i64 + k as i64);
            graph[k].push(w * 2 * n_i64 + u as i64);
        }

        while let Some(c) = queue.pop() {
            let i = (-c % n_i64) as usize;

            if i == visited.len() - 1 {
                return (-c / n_i64) as i32;
            }

            if visited[i] == 0 {
                visited[i] = 1;
                for x in &graph[i] {
                    queue.push(c - c % n_i64 - x);
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_1() {
        let input = vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]];
        let result = Solution::min_cost(4, input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_min_cost_2() {
        let input = vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]];
        let result = Solution::min_cost(4, input);
        assert_eq!(result, 3);
    }
}
