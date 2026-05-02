#include <algorithm>
#include <cassert>
#include <climits>
#include <cstdlib>
#include <iostream>
#include <vector>

using namespace std;

int dp[500][500][3];  // DP array

class Solution {
 private:
  static int func(int i, int j, int k, vector<vector<int>>& coins) {
    if (i == 0 && j == 0) {
      if (k > 0) {
        return max(0, coins[0][0]);
      }
      return coins[0][0];
    }
    if (i < 0 || j < 0 || k < 0) {
      return -1e9;
    }

    if (dp[i][j][k] != INT_MIN) {
      return dp[i][j][k];
    }

    const int x = coins[i][j];
    int ans = x + max(func(i - 1, j, k, coins), func(i, j - 1, k, coins));

    if (x < 0 && k > 0) {
      ans = max(
          {ans, func(i - 1, j, k - 1, coins), func(i, j - 1, k - 1, coins)});
    }

    return dp[i][j][k] = ans;
  }

 public:
  static int maximumAmount(vector<vector<int>>& coins) {
    const int outer_size = coins.size(), inner_size = coins[0].size();
    // Initialize DP with INT_MIN
    fill(&dp[0][0][0], &dp[0][0][0] + 500 * 500 * 3, INT_MIN);
    return func(outer_size - 1, inner_size - 1, 2, coins);
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<vector<int>> coins = {{0, 1, -1}, {1, -2, 3}, {2, -3, 4}};
    int result = sol.maximumAmount(coins);
    cout << "Result: " << result << endl;
    assert(result == 8);
  }

  // Example 2
  {
    vector<vector<int>> coins = {{10, 10, 10}, {10, 10, 10}};
    int result = sol.maximumAmount(coins);
    cout << "Result: " << result << endl;
    assert(result == 40);
  }

  return EXIT_SUCCESS;
}
