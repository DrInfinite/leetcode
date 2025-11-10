#include <bits/stdc++.h>

#include "./solutions/medium/3542_minimum-operations-to-convert-all-elements-to-zero.cc"

int main(int argc, char* argv[]) {
  std::vector<int> input = {1, 2, 1, 2, 1, 2};
  int result = Solution::minOperations(input);

  std::cout << result << std::endl;
  return 0;
}
