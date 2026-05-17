#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> rotateGrid(vector<vector<int>>& grid, int k) {
    int T = 0, L = 0;
    int B = grid.size() - 1, R = grid[0].size() - 1;

    while (T < B && L < R) {
      int len = B - T, wid = R - L;
      int perimeter = 2 * len + 2 * wid;
      int r = k % perimeter;

      while (r--) {
        int tmp = grid[T][L];

        for (int i = L; i < R; i++)
          grid[T][i] = grid[T][i + 1];

        for (int i = T; i < B; i++)
          grid[i][R] = grid[i + 1][R];

        for (int i = R; i > L; i--)
          grid[B][i] = grid[B][i - 1];

        for (int i = B; i > T; i--)
          grid[i][L] = grid[i - 1][L];

        grid[T + 1][L] = tmp;
      }

      T++;
      L++;
      B--;
      R--;
    }

    return grid;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<vector<int>> grid = {{40, 10}, {30, 20}};
    vector<vector<int>> result = sol.rotateGrid(grid, 1);
    cout << "Result:" << endl;
    for (const auto& row : result) {
      for (const auto& item : row)
        cout << item;
      cout << endl;
    }
    cout << endl;
    vector<vector<int>> expected = {{10, 20}, {40, 30}};
    assert(result == expected);
  }

  // Example 2

  {
    vector<vector<int>> grid = {
        {1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}, {13, 14, 15, 16}};
    vector<vector<int>> result = sol.rotateGrid(grid, 2);
    cout << "Result:" << endl;
    for (const auto& row : result) {
      for (const auto& item : row)
        cout << item;
      cout << endl;
    }
    cout << endl;
    vector<vector<int>> expected = {
        {3, 4, 8, 12}, {2, 11, 10, 16}, {1, 7, 6, 15}, {5, 9, 13, 14}};
    assert(result == expected);
  }

  return EXIT_SUCCESS;
}
