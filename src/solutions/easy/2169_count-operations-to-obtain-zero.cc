#include <bits/stdc++.h>

class Solution {
 public:
  static int countOperations(int num1, int num2) {
    int count = 0;

    while (num1 && num2) {
      count += num1 / num2;
      num1 %= num2;
      std::swap(num1, num2);
    }

    return count;
  }
};
