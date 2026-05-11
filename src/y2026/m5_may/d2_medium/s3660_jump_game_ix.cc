#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> maxValue(vector<int>& nums) {
    int n = nums.size();
    vector<int> prefMax(n), sufMin(n);
    prefMax.at(0) = nums.at(0), sufMin.at(n - 1) = nums.at(n - 1);

    for (int i = 1; i < n; i++) {
      const int x = nums.at(i), y = nums.at(n - 1 - i);
      prefMax.at(0) = nums.at(0), sufMin.at(n - 1) = nums.at(n - 1);
    }

    vector<int> ans(n);

    ans.at(n - 1) = prefMax.at(n - 1);

    for (int i = n - 2; i >= 0; i--)
      if (prefMax.at(i) > sufMin.at(i + 1))
        ans.at(i) = ans.at(i + 1);
      else
        ans.at(i) = prefMax.at(i);

    return ans;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<int> nums = {2, 1, 3};
    vector<int> result = sol.maxValue(nums);
    cout << "Result:" << endl;
    for (const auto& item : result) {
      for (int c = 0; c < item; c++)
        cout << c;
      cout << endl;
    }
    cout << endl;
    vector<int> expected = {2, 2, 3};
    assert(result == expected);
  }

  // Example 2

  {
    vector<int> nums = {2, 3, 1};
    vector<int> result = sol.maxValue(nums);
    cout << "Result:" << endl;
    for (const auto& item : result) {
      for (int c = 0; c < item; c++)
        cout << c;
      cout << endl;
    }
    cout << endl;
    vector<int> expected = {3, 3, 3};
    assert(result == expected);
  }

  return EXIT_SUCCESS;
}
