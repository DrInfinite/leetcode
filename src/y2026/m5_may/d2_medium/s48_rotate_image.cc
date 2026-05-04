#include <cstdlib>
#include <vector>

using namespace std;

class Solution {
 public:
  void rotate(vector<vector<int>>& matrix) {
    int n = matrix.size(), k = n - 1;
    for (int i = 0; i < n >> 1; i++)
      for (int j = i; j < k - i; j++) {
        int t = matrix[i][j];
        matrix[i][j] = matrix[k - j][i];
        matrix[k - j][i] = matrix[k - i][k - j];
        matrix[k - i][k - j] = matrix[j][k - i];
        matrix[j][k - i] = t;
      }
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<vector<int>> matrix = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    // no result as rotation is in place
  }

  // Example 2
  {
    vector<vector<int>> matrix = {
        {5, 1, 9, 11}, {2, 4, 8, 10}, {13, 3, 6, 7}, {15, 14, 12, 16}};
    // no result as rotation is in place
  }

  return EXIT_SUCCESS;
}
