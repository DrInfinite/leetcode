long long distributeCandies(int n, int limit) {
  long long count = 0;

  for (int temp = 0; temp <= limit; temp++) {
    int low = n - temp - limit;
    int high = n - temp;

    if (high < 0) {
      continue;
    }

    if (low < 0) {
      low = 0;
    }

    if (high > limit) {
      high = limit;
    }

    if (high >= low) {
      count += high - low + 1;
    }
  }

  return count;
}
