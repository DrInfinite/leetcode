#include <cassert>
#include <iostream>
#include <utility>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<char>> rotateTheBox(vector<vector<char>>& boxGrid) {
    int m = boxGrid.size(), n = boxGrid.at(0).size();

    for (int i = 0; i < m; i++) {
      int emptyPos = n - 1;

      for (int j = n - 1; j >= 0; j--)
        if (boxGrid.at(i).at(j) == '*')
          emptyPos = j - 1;
        else if (boxGrid.at(i).at(j) == '#') {
          swap(boxGrid.at(i).at(j), boxGrid.at(i).at(emptyPos));
          emptyPos--;
        }
    }

    vector<vector<char>> result(n, vector<char>(m));

    for (int i = 0; i < m; i++)
      for (int j = 0; j < n; j++)
        result.at(j).at(m - 1 - i) = boxGrid.at(i).at(j);

    return result;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<vector<char>> boxGrid = {{'#', '.', '#'}};
    vector<vector<char>> result = sol.rotateTheBox(boxGrid);
    cout << "Result:" << endl;
    for (const auto& row : result) {
      for (char c : row)
        cout << c;
      cout << endl;
    }
    cout << endl;
    vector<vector<char>> expected = {{'.'}, {'#'}, {'#'}};
    assert(result == expected);
  }

  // Example 2
  {
    vector<vector<char>> boxGrid = {{'#', '.', '*', '.'}, {'#', '#', '*', '.'}};
    vector<vector<char>> result = sol.rotateTheBox(boxGrid);
    cout << "Result:" << endl;
    for (const auto& row : result) {
      for (char c : row)
        cout << c;
      cout << endl;
    }
    cout << endl;
    vector<vector<char>> expected = {
        {'#', '.'}, {'#', '#'}, {'*', '*'}, {'.', '.'}};
    assert(result == expected);
  }

  // Example 3
  {
    vector<vector<char>> boxGrid = {{'#', '#', '*', '.', '*', '.'},
                                    {'#', '#', '#', '*', '.', '.'},
                                    {'#', '#', '#', '.', '#', '.'}};
    vector<vector<char>> result = sol.rotateTheBox(boxGrid);
    cout << "Result:" << endl;
    for (const auto& row : result) {
      for (char c : row)
        cout << c;
      cout << endl;
    }
    cout << endl;
    vector<vector<char>> expected = {{'.', '#', '#'}, {'.', '#', '#'},
                                     {'#', '#', '*'}, {'#', '*', '.'},
                                     {'#', '.', '*'}, {'#', '.', '.'}};
    assert(result == expected);
  }

  return EXIT_SUCCESS;
}
