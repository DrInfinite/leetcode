#include <cassert>
#include <cstddef>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  int xorAfterQueries(vector<int>& nums, vector<vector<int>>& queries) {
    const int MOD = 1e9 + 7;

    for (size_t i = 0; i < queries.size(); i++) {
      int l = queries[i][0], r = queries[i][1], k = queries[i][2],
          v = queries[i][3];

      for (int idx = l; idx <= r; idx += k) {
        nums[idx] = (1LL * nums[idx] * v) % MOD;
      }
    }

    int ans = 0;
    for (size_t i = 0; i < nums.size(); i++) {
      ans ^= nums[i];
    }

    return ans;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<int> nums = {1, 1, 1};
    vector<vector<int>> queries = {{0, 2, 1, 4}};
    int result = sol.xorAfterQueries(nums, queries);
    cout << "Result: " << result << endl;
    assert(result == 4);
  }

  // Example 2
  {
    vector<int> nums = {2, 3, 1, 5, 4};
    vector<vector<int>> queries = {{1, 4, 2, 3}, {0, 2, 1, 2}};
    int result = sol.xorAfterQueries(nums, queries);
    cout << "Result: " << result << endl;
    assert(result == 31);
  }

  return EXIT_SUCCESS;
}
