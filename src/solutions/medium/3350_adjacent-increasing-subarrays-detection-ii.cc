#include <bits/stdc++.h>

class Solution {
 public:
  static int maxIncreasingSubarrays(std::vector<int>& nums) {
    int n = nums.size(), ans = 0, count = 1, precount = 0;

    for (int i = 1; i < n; i++) {
      if (nums[i] > nums[i - 1]) {
        ++count;
      } else {
        precount = count;
        count = 1;
      }
      ans = std::max(ans, std::min(precount, count));
      ans = std::max(ans, count / 2);
    }

    return ans;
  }
};
