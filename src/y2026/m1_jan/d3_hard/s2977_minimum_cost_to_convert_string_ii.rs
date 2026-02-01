// LeetCode Problem 2977. Minimum Cost to Convert String II
// Difficulty: Hard

// Time Complexity: O(m^3 + n^2) — Floyd–Warshall runs on O(m) unique transformation strings and DP checks all substrings.
// Space Complexity: O(m^2) — dominated by the all-pairs shortest-path distance matrix.

#![allow(dead_code)]
struct Solution;

const INF: i32 = 0x3f3f3f3f;

struct TrieNode {
    child: [u16; 26], // Use indices instead of Box pointers
    id: i16,
}

struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode {
                child: [0; 26],
                id: -1,
            }],
        }
    }

    #[inline]
    fn add(&mut self, word: &[u8], node_id: &mut i16) -> i16 {
        let mut curr = 0u16;

        for &ch in word {
            let idx = (ch - b'a') as usize;
            let child_idx = self.nodes[curr as usize].child[idx];

            if child_idx == 0 {
                let new_idx = self.nodes.len() as u16;
                self.nodes.push(TrieNode {
                    child: [0; 26],
                    id: -1,
                });
                self.nodes[curr as usize].child[idx] = new_idx;
                curr = new_idx;
            } else {
                curr = child_idx;
            }
        }

        if self.nodes[curr as usize].id == -1 {
            *node_id += 1;
            self.nodes[curr as usize].id = *node_id;
        }

        self.nodes[curr as usize].id
    }

    #[inline]
    fn traverse(&self, start: u16, ch: u8) -> u16 {
        self.nodes[start as usize].child[(ch - b'a') as usize]
    }

    #[inline]
    fn get_id(&self, idx: u16) -> i16 {
        self.nodes[idx as usize].id
    }
}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let n = source.len();
        let m = original.len();

        // Build Trie
        let mut trie = Trie::new();
        let mut node_id = -1i16;

        // Flat distance matrix (better cache locality)
        let capacity = m * 2;
        let mut dist = vec![INF; capacity * capacity];

        // Initialize diagonal
        for i in 0..capacity {
            dist[i * capacity + i] = 0;
        }

        // Build graph
        for i in 0..m {
            let x = trie.add(original[i].as_bytes(), &mut node_id) as usize;
            let y = trie.add(changed[i].as_bytes(), &mut node_id) as usize;
            let idx = x * capacity + y;
            dist[idx] = dist[idx].min(cost[i]);
        }

        let size = (node_id + 1) as usize;

        // Floyd-Warshall with early termination
        for k in 0..size {
            let k_base = k * capacity;
            for i in 0..size {
                let ik = dist[i * capacity + k];
                if ik >= INF {
                    continue;
                }

                let i_base = i * capacity;
                for j in 0..size {
                    let kj = dist[k_base + j];
                    if kj < INF {
                        let idx = i_base + j;
                        dist[idx] = dist[idx].min(ik + kj);
                    }
                }
            }
        }

        // DP with byte arrays
        let src = source.as_bytes();
        let tgt = target.as_bytes();
        let mut dp = vec![i64::MAX; n];

        for j in 0..n {
            let base = if j == 0 { 0 } else { dp[j - 1] };

            if base == i64::MAX {
                continue;
            }

            // Single character match
            if src[j] == tgt[j] {
                dp[j] = dp[j].min(base);
            }

            // Try substring transformations
            let mut u_idx = 0u16;
            let mut v_idx = 0u16;

            for i in j..n {
                u_idx = trie.traverse(u_idx, src[i]);
                v_idx = trie.traverse(v_idx, tgt[i]);

                if u_idx == 0 || v_idx == 0 {
                    break;
                }

                let u_id = trie.get_id(u_idx);
                let v_id = trie.get_id(v_idx);

                if u_id >= 0 && v_id >= 0 {
                    let d = dist[u_id as usize * capacity + v_id as usize];
                    if d < INF {
                        let new_val = base + d as i64;
                        dp[i] = dp[i].min(new_val);
                    }
                }
            }
        }

        if dp[n - 1] == i64::MAX {
            -1
        } else {
            dp[n - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::vector_string::vector_string;

    use super::*;

    #[test]
    fn test_minimum_cost_1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = vector_string(vec!["a", "b", "c", "c", "e", "d"]);
        let changed = vector_string(vec!["b", "c", "b", "e", "b", "e"]);
        let cost = vec![2, 5, 5, 1, 2, 20];
        let result = Solution::minimum_cost(source, target, original, changed, cost);

        assert_eq!(result, 28);
    }

    #[test]
    fn test_minimum_cost_2() {
        let source = "abcdefgh".to_string();
        let target = "acdeeghh".to_string();
        let original = vector_string(vec!["bcd", "fgh", "thh"]);
        let changed = vector_string(vec!["cde", "thh", "ghh"]);
        let cost = vec![1, 3, 5];
        let result = Solution::minimum_cost(source, target, original, changed, cost);

        assert_eq!(result, 9);
    }
}
