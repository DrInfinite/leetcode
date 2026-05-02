#include <algorithm>
#include <cassert>
#include <cstdlib>
#include <numeric>
#include <stack>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> survivedRobotsHealths(vector<int>& positions,
                                    vector<int>& healths, string directions) {
    int n = positions.size();
    vector<int> order(n);
    iota(order.begin(), order.end(), 0);
    sort(order.begin(), order.end(),
         [&](int a, int b) { return positions[a] < positions[b]; });

    stack<int> st;
    vector<bool> dead(n, false);

    for (int i : order) {
      if (directions[i] == 'R') {
        st.push(i);
      } else {
        while (!st.empty() && directions[st.top()] == 'R') {
          int top = st.top();

          if (healths[top] > healths[i]) {
            healths[top]--;
            dead[i] = true;
            break;
          } else if (healths[top] < healths[i]) {
            healths[i]--;
            dead[top] = true;
            st.pop();
          } else {
            dead[i] = true;
            dead[top] = true;
            st.pop();
            break;
          }
        }
        if (!dead[i]) {
          st.push(i);
        }
      }
    }

    vector<int> result;
    for (int i = 0; i < n; i++) {
      if (!dead[i]) {
        result.push_back(healths[i]);
      }
    }

    return result;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<int> positions = {5, 4, 3, 2, 1};
    vector<int> healths = {2, 17, 9, 15, 10};
    string directions = "RRRRR";
    vector<int> result =
        sol.survivedRobotsHealths(positions, healths, directions);
    vector<int> expected = {2, 17, 9, 15, 10};
    assert(result == expected);
  }

  // Example 2
  {
    vector<int> positions = {3, 5, 2, 6};
    vector<int> healths = {10, 10, 15, 12};
    string directions = "RLRL";
    vector<int> result =
        sol.survivedRobotsHealths(positions, healths, directions);
    vector<int> expected = {14};
    assert(result == expected);
  }

  // Example 3
  {
    vector<int> positions = {1, 2, 5, 6};
    vector<int> healths = {10, 10, 11, 11};
    string directions = "RLRL";
    vector<int> result =
        sol.survivedRobotsHealths(positions, healths, directions);
    vector<int> expected = {};
    assert(result == expected);
  }

  return EXIT_SUCCESS;
}
