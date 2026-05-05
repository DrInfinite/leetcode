#include <cassert>
#include <iostream>
#include <string>

using namespace std;

class Solution {
 public:
  bool judgeCircle(string moves) {
    int right = 0, left = 0, up = 0, down = 0;

    for (char move : moves) {
      if (move == 'R')
        right++;
      else if (move == 'L')
        left++;
      else if (move == 'U')
        up++;
      else
        down++;
    }

    if (right == left && up == down)
      return true;

    return false;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    string moves = "UD";
    bool result = sol.judgeCircle(moves);
    cout << "Result: " << result << endl;
    assert(result == true);
  }

  // Example 2
  {
    string moves = "LL";
    bool result = sol.judgeCircle(moves);
    cout << "Result: " << result << endl;
    assert(result == false);
  }

  return EXIT_SUCCESS;
}
