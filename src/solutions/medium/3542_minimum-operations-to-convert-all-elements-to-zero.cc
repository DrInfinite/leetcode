#include <vector>

class Solution {
 public:
  static int minOperations(std::vector<int>& nums) {
    std::vector<int> stack;
    int result = 0;

    for (int i : nums) {
      while (!stack.empty() && stack.back() > i) {
        stack.pop_back();
      }
      if (i == 0)
        continue;
      if (stack.empty() || stack.back() < i) {
        ++result;
        stack.push_back(i);
      }
    }

    return result;
  }
};
