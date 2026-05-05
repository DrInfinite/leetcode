#include <cassert>
#include <iostream>
#include <string>

using namespace std;

class Solution {
 public:
  string decodeCiphertext(string encodedText, int rows) {
    if (rows == 1)
      return encodedText;

    int n = encodedText.size(), cols = n / rows;

    string result;
    result.reserve(n);

    for (int j = 0; j < cols; ++j) {
      int row = 0, col = j;

      while (row < rows && col < cols) {
        result += encodedText[row * cols + col];
        ++row;
        ++col;
      }
    }

    while (!result.empty() && result.back() == ' ')
      result.pop_back();

    return result;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    string encodedText = "iveo    eed   l te   olc";
    int rows = 4;
    string result = sol.decodeCiphertext(encodedText, rows);
    string expected = "i love leetcode";
    cout << "Result: " << result << endl;
    assert(result == expected);
  }

  // Example 2
  {
    string encodedText = "ch   ie   pr";
    int rows = 3;
    string result = sol.decodeCiphertext(encodedText, rows);
    string expected = "cipher";
    cout << "Result: " << result << endl;
    assert(result == expected);
  }

  // Example 3
  {
    string encodedText = "coding";
    int rows = 1;
    string result = sol.decodeCiphertext(encodedText, rows);
    string expected = "coding";
    cout << "Result: " << result << endl;
    assert(result == expected);
  }

  return EXIT_SUCCESS;
}
