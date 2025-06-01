/**
 * @brief Distribute Candies Among Children II
 *
 * This function calculates the total number of ways to distribute n candies
 * among 3 children such that no child gets more than limit candies.
 *
 * @param n The total number of candies to distribute.
 * @param limit The maximum number of candies each child can receive.
 * @return The total number of ways to distribute the candies.
 *
 * @example
 * Input: n = 5, limit = 2
 * Output: 3
 * Explanation: There are 3 ways to distribute 5 candies such that no child gets
 * more than 2 candies: (1, 2, 2), (2, 1, 2) and (2, 2, 1).
 *
 * @example
 * Input: n = 3, limit = 3
 * Output: 10
 * Explanation: There are 10 ways to distribute 3 candies such that no child
 * gets more than 3 candies: (0, 0, 3), (0, 1, 2), (0, 2, 1), (0, 3, 0), (1, 0,
 * 2), (1, 1, 1), (1, 2, 0), (2, 0, 1), (2, 1, 0) and (3, 0, 0).
 *
 * @constraints
 * 1 <= n <= 10^6
 * 1 <= limit <= 10^6
 */
inline long long distributeCandies(int n, int limit) {
  long long count = 0;

  for (int temp = 0; temp <= limit; temp++) {
    int low = n - temp - limit;
    int high = n - temp;

    if (high < 0) {
      continue;
    }

    if (low < 0) {
      low = 0;
    }

    if (high > limit) {
      high = limit;
    }

    if (high >= low) {
      count += high - low + 1;
    }
  }

  return count;
}
