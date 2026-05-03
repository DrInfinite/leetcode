
#include <algorithm>
#include <cassert>
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxWalls(vector<int>& r, vector<int>& d, vector<int>& w) {
    int n = r.size();

    vector<pair<int, int>> rd(n);
    for (int i = 0; i < n; i++)
      rd[i] = {r[i], d[i]};

    sort(rd.begin(), rd.end());
    sort(w.begin(), w.end());

    vector<int> walls;
    int base = 0, j = 0;
    size_t i = 0;

    while (i < w.size() && j < n) {
      if (w[i] < rd[j].first)
        walls.push_back(w[i++]);
      else if (w[i] == rd[j].first)
        base++, i++;
      else
        j++;
    }
    while (i < w.size())
      walls.push_back(w[i++]);

    auto cntL = [&](int l, int r, int s, int e) {
      return upper_bound(walls.begin() + s, walls.begin() + e, r) -
             lower_bound(walls.begin() + s, walls.begin() + e, l);
    };

    size_t idx = 0;
    while (idx < walls.size() && walls[idx] < rd[0].first)
      idx++;

    int dpL = cntL(rd[0].first - rd[0].second, rd[0].first, 0, idx);
    int dpR = 0;

    for (int k = 1; k < n; k++) {
      size_t s = idx;
      while (idx < walls.size() && walls[idx] < rd[k].first)
        idx++;
      int e = idx;

      long long leftReach = (long long)rd[k].first - rd[k].second;
      long long rightReach = (long long)rd[k - 1].first + rd[k - 1].second;

      int hitL = cntL(leftReach, rd[k].first, s, e);
      int hitR = cntL(rd[k - 1].first, rightReach, s, e);

      int both = (rightReach >= leftReach) ? (e - s) : (hitL + hitR);

      int newL = max(dpL + hitL, dpR + both);
      int newR = max(dpL, dpR + hitR);

      dpL = newL;
      dpR = newR;
    }

    int s = idx, e = walls.size();
    long long lastReach = (long long)rd[n - 1].first + rd[n - 1].second;

    int lastHit = upper_bound(walls.begin() + s, walls.begin() + e, lastReach) -
                  (walls.begin() + s);

    return max(dpL, dpR + lastHit) + base;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<int> robots = {4};
    vector<int> distance = {3};
    vector<int> walls = {1, 10};
    int result = sol.maxWalls(robots, distance, walls);
    cout << "Result: " << result << endl;
    assert(result == 1);
  }

  // Example 2
  {
    vector<int> robots = {10, 2};
    vector<int> distance = {5, 1};
    vector<int> walls = {5, 2, 7};
    int result = sol.maxWalls(robots, distance, walls);
    cout << "Result: " << result << endl;
    assert(result == 3);
  }

  // Example 3
  {
    vector<int> robots = {1, 2};
    vector<int> distance = {100, 1};
    vector<int> walls = {10};
    int result = sol.maxWalls(robots, distance, walls);
    cout << "Result: " << result << endl;
    assert(result == 0);
  }

  return EXIT_SUCCESS;
}
