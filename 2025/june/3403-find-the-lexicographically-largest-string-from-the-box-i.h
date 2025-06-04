/**
 * @brief Finds the lexicographically largest string from the box after all
 * rounds are finished.
 *
 * @param word The input string to be split into substrings.
 * @param numFriends The number of friends playing the game, which determines
 * the number of substrings to split the word into.
 *
 * @return The lexicographically largest string from the box after all rounds
 * are finished.
 *
 * @details This function simulates a game where the input string is split into
 * numFriends non-empty substrings in each round, and all the split substrings
 * are put into a box. The function returns the lexicographically largest string
 * from the box after all possible splits have been made.
 *
 * @example
 *      Input: word = "dbca", numFriends = 2
 *      Output: "dbc"
 *
 * @example
 *      Input: word = "gggg", numFriends = 4
 *      Output: "g"
 *
 * @note The input string consists only of lowercase English letters, and its
 * length is between 1 and 5 * 10^3. The number of friends is between 1 and the
 * length of the input string.
 */
#include <string.h>
#define MAX(a, b) a > b ? a : b
#define MIN(a, b) a > b ? b : a

inline char *answerString(char *word, int numFriends) {
  if (numFriends == 1) {
    return word;
  }

  int N = strlen(word), maxIndex = 0;

  for (int index = 1; index < N;) {
    int lenEq = 0;

    while (index + lenEq < N && word[index + lenEq] == word[lenEq + maxIndex]) {
      lenEq++;
    }

    if (index + lenEq < N && word[index + lenEq] > word[lenEq + maxIndex]) {
      int temp = index;
      index = MAX(maxIndex + lenEq + 1, index + 1);
      maxIndex = temp;
    } else {
      index = index + lenEq + 1;
    }
  }

  int lenRs = MIN(N - (numFriends - 1), N - maxIndex);
  word[maxIndex + lenRs] = 0;

  return &word[maxIndex];
}
