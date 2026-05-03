#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>

using namespace std;

class Solution {
 public:
  bool rotateString(string s, string goal) {
    if (s.size() != goal.size())
      return false;

    for (size_t i = 0; i < s.size(); i++) {
      rotate(s.begin(), s.begin() + 1, s.end());
      if (s == goal)
        return true;
    }

    return false;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    string s = "abcde";
    string goal = "cdeab";
    int result = sol.rotateString(s, goal);
    cout << "Result: " << result << endl;
    assert(result == true);
  }

  // Example 2
  {
    string s = "abcde";
    string goal = "abced";
    int result = sol.rotateString(s, goal);
    cout << "Result: " << result << endl;
    assert(result == false);
  }

  return EXIT_SUCCESS;
}
