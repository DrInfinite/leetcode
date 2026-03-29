// LeetCode Problem 1622. Fancy Sequence
// Difficulty: Hard

// Time Complexity
// Constructor (new): O(1)
// append(val): O(log MOD) — dominated by the modular inverse computation via mod_pow, which uses fast exponentiation.
// add_all(inc): O(1)
// mult_all(m): O(1)
// get_index(idx): O(1)

// Space Complexity: O(n) — where $ n $ is the number of elements appended to the sequence.

#![allow(dead_code)]
const MOD: i64 = 1_000_000_007;

struct Fancy {
    seq: Vec<i64>,
    add: i64,
    mul: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Fancy {
            seq: Vec::new(),
            add: 0,
            mul: 1,
        }
    }

    fn append(&mut self, val: i32) {
        let v = ((val as i64 - self.add).rem_euclid(MOD) * Self::mod_pow(self.mul, MOD - 2)) % MOD;
        self.seq.push(v);
    }

    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64).rem_euclid(MOD);
    }

    fn mult_all(&mut self, m: i32) {
        self.mul = (self.mul * m as i64).rem_euclid(MOD);
        self.add = (self.add * m as i64).rem_euclid(MOD);
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;

        if idx >= self.seq.len() {
            return -1;
        }

        ((self.seq[idx] * self.mul + self.add).rem_euclid(MOD)) as i32
    }

    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut result: i64 = 1;
        base %= MOD;

        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            exp >>= 1;
            base = base * base % MOD;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fancy_sequence_1() {
        let mut fancy = Fancy::new();
        fancy.append(2); // seq: [2]
        fancy.add_all(3); // seq: [5]
        fancy.append(7); // seq: [5, 7]
        fancy.mult_all(2); // seq: [10, 14]
        assert_eq!(fancy.get_index(0), 10); // ✅ returns 10
        fancy.add_all(3); // seq: [13, 17]
        fancy.append(10); // seq: [13, 17, 10]
        fancy.mult_all(2); // seq: [26, 34, 20]
        assert_eq!(fancy.get_index(0), 26); // ✅ returns 26
        assert_eq!(fancy.get_index(1), 34); // ✅ returns 34
        assert_eq!(fancy.get_index(2), 20); // ✅ returns 20
    }
}
