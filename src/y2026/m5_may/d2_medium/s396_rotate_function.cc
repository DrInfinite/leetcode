#include <cassert>
#include <cstdlib>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxRotateFunction(vector<int>& nums) {
    int sum = 0, F = 0;
    int n = nums.size();

    for (int i = 0; i < n; i++) {
      sum += nums[i];
      F += i * nums[i];
    }

    int result = F;

    for (int i = 1; i < n; i++) {
      F += sum - n * nums[n - i];
      result = max(result, F);
    }

    return result;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<int> nums = {4, 3, 2, 6};
    int result = sol.maxRotateFunction(nums);
    cout << "Result: " << result << endl;
    assert(result == 26);
  }

  // Example 2
  {
    vector<int> nums = {100};
    int result = sol.maxRotateFunction(nums);
    cout << "Result: " << result << endl;
    assert(result == 0);
  }

  return EXIT_SUCCESS;
}
