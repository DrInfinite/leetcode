// LeetCode Problem 3510. Minimum Pair Removal to Sort Array II
// Difficulty: Hard

// Time Complexity: O(n . log(n)) - n is the size of the binary heap and nums vector

// Space Complexity: O(n) - n is the max size of all vectors, binary heaps and priority queues

#![allow(dead_code)]

use std::{cmp::Ordering, collections::BinaryHeap};
struct Solution;

#[derive(Debug)]
struct Node {
    value: i64,
    left: usize,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    first: usize,
    second: usize,
    cost: i64,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost == other.cost {
            self.first.cmp(&other.first).reverse()
        } else {
            self.cost.cmp(&other.cost).reverse()
        }
    }
}

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nodes = Vec::with_capacity(n);
        let mut merged = vec![false; n];
        let mut pq = BinaryHeap::new();

        let mut decrease_count = 0;
        let mut count = 0;

        nodes.push(Node {
            value: nums[0] as i64,
            left: 0,
            prev: None,
            next: Some(1),
        });

        (1..n).for_each(|i| {
            nodes.push(Node {
                value: nums[i] as i64,
                left: i,
                prev: Some(i - 1),
                next: if i < n - 1 { Some(i + 1) } else { None },
            });

            if i > 0 {
                nodes[i - 1].next = Some(i);
                pq.push(Item {
                    first: i - 1,
                    second: i,
                    cost: nums[i - 1] as i64 + nums[i] as i64,
                });

                if nums[i - 1] > nums[i] {
                    decrease_count += 1;
                }
            }
        });

        while decrease_count > 0 {
            let Item {
                first: first_idx,
                second: second_idx,
                cost,
            } = pq.pop().unwrap();

            if merged[first_idx]
                || merged[second_idx]
                || nodes[first_idx].value + nodes[second_idx].value != cost
            {
                continue;
            }
            count += 1;
            if nodes[first_idx].value > nodes[second_idx].value {
                decrease_count -= 1;
            }

            let prev_idx = nodes[first_idx].prev;
            let next_idx = nodes[second_idx].next;
            nodes[first_idx].next = next_idx;
            if let Some(next) = next_idx {
                nodes[next].prev = Some(first_idx);
            }

            if let Some(prev) = prev_idx {
                if nodes[prev].value > nodes[first_idx].value && nodes[prev].value <= cost {
                    decrease_count -= 1;
                } else if nodes[prev].value <= nodes[first_idx].value && nodes[prev].value > cost {
                    decrease_count += 1;
                }

                pq.push(Item {
                    first: prev,
                    second: first_idx,
                    cost: nodes[prev].value + cost,
                });
            }

            if let Some(next) = next_idx {
                if nodes[second_idx].value > nodes[next].value && cost <= nodes[next].value {
                    decrease_count -= 1;
                } else if nodes[second_idx].value <= nodes[next].value && cost > nodes[next].value {
                    decrease_count += 1;
                }

                pq.push(Item {
                    first: first_idx,
                    second: next,
                    cost: cost + nodes[next].value,
                });
            }

            nodes[first_idx].value = cost;
            merged[second_idx] = true;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_pair_removal_1() {
        let input = vec![5, 2, 3, 1];
        let result = Solution::minimum_pair_removal(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_minimum_pair_removal_2() {
        let input = vec![1, 2, 2];
        let result = Solution::minimum_pair_removal(input);
        assert_eq!(result, 0);
    }
}
