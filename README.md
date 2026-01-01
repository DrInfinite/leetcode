# LeetCode Solutions in Rust

A personal repository of LeetCode problem solutions written in Rust. This project serves as both a learning resource for problem-solving techniques and a public showcase of algorithmic thinking and implementation skills.

## Table of Contents

- [Legal Notice & Disclaimers](#legal-notice--disclaimers)
- [Project Purpose](#project-purpose)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [How to Use](#how-to-use)
- [Adding New Solutions](#adding-new-solutions)
- [Solution Format](#solution-format)
- [Future Plans](#future-plans)
- [License](#license)

## Legal Notice & Disclaimers

### Intellectual Property

**This repository contains personal solutions to LeetCode problems.** The problem statements, test cases, and problem descriptions are the intellectual property of LeetCode and are used solely for educational and learning purposes.

### Important Notes for Users

- **Solve problems yourself first**: This repository is intended as a reference after you've attempted to solve problems on your own. Viewing solutions before attempting problems defeats the educational purpose.
- **LeetCode's Terms of Service**: Users of this repository should respect LeetCode's Terms of Service and only use these solutions in compliance with their policies.
- **Educational Use Only**: These solutions are shared for educational purposes to demonstrate problem-solving approaches and Rust implementations.
- **No Guarantee of Optimality**: While solutions aim for good time and space complexity, there may be more optimal approaches available.

### Attribution

If you reference or adapt solutions from this repository, please:

- Credit the original LeetCode problem
- Acknowledge this repository if sharing publicly
- Understand that you're responsible for ensuring your usage complies with LeetCode's terms

---

## Project Purpose

This project serves dual purposes:

1. **Personal Learning**: A structured approach to improving problem-solving skills, understanding different algorithmic patterns, and deepening Rust language proficiency.
2. **Public Showcase**: Demonstrating algorithmic thinking, code quality, and the ability to solve problems across varying difficulty levels.

---

## Project Structure

```
leetcode/
├── Cargo.toml              # Project manifest and dependencies
├── README.md               # This file
├── LICENSE                 # ISC License
├── templates/
│   └── test-case.md       # Template for documenting test cases
└── src/
    ├── main.rs            # Entry point
    └── y2026/             # Solutions organized by year
        ├── mod.rs
        ├── m1_jan/        # January problems
        │   ├── mod.rs
        │   ├── d1_easy/   # Difficulty level: Easy
        │   ├── d2_medium/ # Difficulty level: Medium
        │   └── d3_hard/   # Difficulty level: Hard
        ├── m2_feb/        # February problems
        │   ├── mod.rs
        │   ├── d1_easy/
        │   ├── d2_medium/
        │   └── d3_hard/
        └── ... (more months)
```

### Naming Convention

- **Year directories**: `y{YYYY}` (e.g., `y2026` for 2026)
- **Month directories**: `m{M}_{MONTH}` (e.g., `m1_jan`, `m2_feb`)
- **Difficulty directories**: `d{LEVEL}_{DIFFICULTY}` (e.g., `d1_easy`, `d2_medium`, `d3_hard`)
- **Solution files**: `s{PROBLEM_NUMBER}_{PROBLEM_NAME}.rs` (e.g., `s66_plus_one.rs`)

---

## Getting Started

### Prerequisites

- **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)
- **Cargo**: Comes bundled with Rust

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd leetcode
   ```

2. Verify the setup:

   ```bash
   cargo build
   ```

3. Run the project:

   ```bash
   cargo run
   ```

---

## How to Use

### Running Individual Solutions

Each solution file is a standalone module. To test a specific solution:

1. Navigate to the appropriate solution file in the directory structure
2. Review the implementation and complexity analysis in comments
3. Study the approach and understand the logic

### Browsing Solutions

Solutions are organized chronologically and by difficulty level, making it easy to:

- Find problems solved on a specific date
- Filter by difficulty level
- Track progress over time

---

## Adding New Solutions

To add a new solution, follow these steps:

1. **Identify the problem**: Determine the LeetCode problem number and name, the month, and difficulty level

2. **Create the file**: Add a new file in the appropriate directory following the naming convention:

   ```
   src/y2026/m{MONTH}/d{LEVEL}_{DIFFICULTY}/s{PROBLEM_NUMBER}_{PROBLEM_NAME}.rs
   ```

3. **Write the solution**: Structure your solution with the following format:

   ```rust
   // LeetCode Problem {NUMBER}. {PROBLEM_NAME}
   // Difficulty: {EASY|MEDIUM|HARD}
   //
   // Time Complexity: O(...)
   // Space Complexity: O(...)

   impl Solution {
    pub fn solution_function(input: Type) -> ReturnType {
       // Implementation
    }
   }
   ```

4. **Update module files**: Add the module declaration to the parent `mod.rs` file:

   ```rust
   pub mod s{PROBLEM_NUMBER}_{PROBLEM_NAME};
   ```

5. **Document in test-case.md**: If desired, add test cases and expected outputs to `templates/test-case.md` for reference

---

## Solution Format

_NOTE: The below example is only for illustrative purposes._

Each solution includes:

### 1. **Header Comment**

```rust
// LeetCode Problem 66. Plus One
// Difficulty: Easy
//
// Time Complexity: O(n) - where n is the length of the array
// Space Complexity: O(1) - only using a constant amount of extra space
```

### 2. **Complexity Analysis**

- **Time Complexity**: Documented with Big O notation
- **Space Complexity**: Documented with Big O notation (excluding input/output space)

### 3. **Clean, Idiomatic Rust Code**

Solutions aim to be:

- Readable
- Idiomatic Rust (following Rust conventions)
- Efficient without sacrificing clarity

---

## Future Plans

- [ ] Complete all problems for 2026
- [ ] Expand to multiple years
- [ ] Add alternative solutions for some problems
- [ ] Potentially open for community contributions (see Contributions section below)

### Future Contributions

Currently, this is a personal project. However, contributions may be accepted in the future under the following conditions:

- Solutions must strictly follow the project's structure and naming conventions
- All solutions must include proper complexity analysis
- Solutions should be idiomatic Rust
- Contributions should respect LeetCode's Terms of Service and intellectual property
- If I feel like it ;)

---

## License

This project is licensed under the **ISC License**.

For the full license text, see [LICENSE](LICENSE)

---

## Support & Questions

For questions about specific solutions or the project structure, feel free to reach out or open an issue.

**Happy coding!** 🚀
