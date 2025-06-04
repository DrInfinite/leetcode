#include <string.h>

/**
 * @brief Returns the maximum number of candies that can be collected from the
 * given boxes.
 *
 * You have n boxes labeled from 0 to n - 1. You are given four arrays: status,
 * candies, keys, and containedBoxes where:
 * - status[i] is 1 if the ith box is open and 0 if the ith box is closed,
 * - candies[i] is the number of candies in the ith box,
 * - keys[i] is a list of the labels of the boxes you can open after opening the
 * ith box.
 * - containedBoxes[i] is a list of the boxes you found inside the ith box.
 *
 * You are given an integer array initialBoxes that contains the labels of the
 * boxes you initially have. You can take all the candies in any open box and
 * you can use the keys in it to open new boxes and you also can use the boxes
 * you find in it.
 *
 * @param status An array indicating the status of each box (open or closed).
 * @param candies An array indicating the number of candies in each box.
 * @param keys An array of lists indicating the boxes that can be opened with
 * the keys found in each box.
 * @param containedBoxes An array of lists indicating the boxes found inside
 * each box.
 * @param initialBoxes An array indicating the labels of the boxes you initially
 * have.
 *
 * @return The maximum number of candies that can be collected.
 */
inline int maxCandies(int *status, int statusSize, int *candies,
                      int candiesSize, int **keys, int keysSize,
                      int *keysColSize, int **containedBoxes,
                      int containedBoxesSize, int *containedBoxesColSize,
                      int *initialBoxes, int initialBoxesSize) {
  int result = 0;
  bool key[statusSize];
  int buffer[statusSize], left = 0, right = 0;

  memset(key, 0, sizeof(key));
  for (int i = 0; i < initialBoxesSize; i++) {
    buffer[right++] = initialBoxes[i];
  }

  while (left < right) {
    int leftMax = right;

    while (left < leftMax) {
      int iBox = buffer[left];

      if (status[iBox] || (!status[iBox] && key[iBox])) {
        result += candies[iBox];
        for (int i = 0; i < containedBoxesColSize[iBox]; i++)
          buffer[right++] = containedBoxes[iBox][i];
        for (int i = 0; i < keysColSize[iBox]; i++)
          key[keys[iBox][i]] = true;
        left++;
      } else {
        leftMax--;
        int tmp = buffer[left];
        buffer[left] = buffer[leftMax];
        buffer[leftMax] = tmp;
      }
    }

    int countDeadLock = 0;
    for (int i = leftMax; i < right; i++)
      if (!status[buffer[i]] && !key[buffer[i]])
        countDeadLock++;
    if (countDeadLock == right - leftMax)
      break;
  }

  return result;
}
