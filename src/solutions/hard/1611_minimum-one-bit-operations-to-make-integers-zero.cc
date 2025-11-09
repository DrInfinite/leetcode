class Solution {
 public:
  static int minimumOneBitOperations(int n) {
    if (n == 0) {
      return 0;
    }

    int k = 0;
    int current = 1;

    while (current * 2 <= n) {
      current *= 2;
      k++;
    }

    return (1 << (k + 1)) - 1 - minimumOneBitOperations(n ^ current);
  }
};
