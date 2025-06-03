/**
 * @brief Returns the minimum number of candies needed to distribute to children
 * based on their ratings.
 *
 * There are n children standing in a line, each with a rating value given in
 * the integer array ratings. The distribution of candies is subject to the
 * following requirements:
 * - Each child must have at least one candy.
 * - Children with a higher rating get more candies than their neighbors.
 *
 * @param ratings An integer array containing the ratings of the children.
 * @return The minimum number of candies needed to distribute to the children.
 *
 * @example
 * Input: ratings = [1,0,2]
 * Output: 5
 * Explanation: You can allocate to the first, second and third child with 2, 1,
 * 2 candies respectively.
 *
 * @example
 * Input: ratings = [1,2,2]
 * Output: 4
 * Explanation: You can allocate to the first, second and third child with 1, 2,
 * 1 candies respectively.
 *
 * @note
 * - n == ratings.length
 * - 1 <= n <= 2 * 10^4
 * - 0 <= ratings[i] <= 2 * 10^4
 */
#include <stdlib.h>

inline int candy(int *ratings, int ratingsSize) {
  int sum = 0;

  if (ratingsSize == 0) {
    return 0;
  }

  int *candies = (int *)malloc(ratingsSize * sizeof(int));

  for (int i = 0; i < ratingsSize; i++) {
    candies[i] = 1;
  }

  for (int i = 1; i < ratingsSize; i++) {
    if (ratings[i] > ratings[i - 1])
      candies[i] = candies[i - 1] + 1;
  }
  for (int i = ratingsSize - 2; i >= 0; i--) {
    if (ratings[i] > ratings[i + 1])
      candies[i] =
          (candies[i] > candies[i + 1] + 1) ? candies[i] : candies[i + 1] + 1;
  }

  for (int i = 0; i < ratingsSize; i++) {
    sum += candies[i];
  }

  free(candies);
  return sum;
}
