#include <cassert>
#include <iostream>

using namespace std;

class Solution {
 public:
  int rotatedDigits(int n) {
    int result = 0;

    for (int i = 1; i <= n; i++) {
      int check = i;
      bool valid = true;
      bool changed = false;

      while (check > 0 && valid) {
        int digit = check % 10;

        if (digit == 3 || digit == 4 || digit == 7)
          valid = false;
        else if (digit == 2 || digit == 5 || digit == 6 || digit == 9)
          changed = true;

        check /= 10;
      }

      if (valid && changed)
        result += 1;
    }

    return result;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    int result = sol.rotatedDigits(10);
    cout << "Result: " << result << endl;
    assert(result == 4);
  }

  // Example 2
  {
    int result = sol.rotatedDigits(1);
    cout << "Result: " << result << endl;
    assert(result == 0);
  }

  // Example 3
  {
    int result = sol.rotatedDigits(2);
    cout << "Result: " << result << endl;
    assert(result == 1);
  }

  return EXIT_SUCCESS;
}
