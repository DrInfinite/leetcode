#include <cassert>
#include <iostream>
#include <vector>
#include "../../../utils/list_node.hpp"

using namespace std;

class Solution {
 public:
  ListNode* rotateRight(ListNode* head, int k) {
    if (head == NULL || head->next == NULL || k == 0)
      return head;
    ListNode* temp = head;
    int len = 1;

    while (temp->next != NULL) {
      temp = temp->next;
      len++;
    }

    k = k % len;
    if (k == 0)
      return head;
    temp->next = head;
    int steps = len - k;

    ListNode* newtemp = head;
    for (int i = 1; i < steps; i++)
      newtemp = newtemp->next;

    ListNode* result = newtemp->next;
    newtemp->next = NULL;

    return result;
  }
};

int main() {
  Solution sol;

  // Example 1
  {
    vector<int> input = {1, 2, 3, 4, 5};
    ListNode* head = listnode_create(input);
    int k = 2;
    ListNode* result = sol.rotateRight(head, k);
    vector<int> output = {4, 5, 1, 2, 3};
    ListNode* expected = listnode_create(output);
    cout << "Result: " << result << endl;
    assert(listnode_compare(result, expected));
  }

  // Example 2
  {
    vector<int> input = {0, 1, 2};
    ListNode* head = listnode_create(input);
    int k = 4;
    ListNode* result = sol.rotateRight(head, k);
    vector<int> output = {2, 0, 1};
    ListNode* expected = listnode_create(output);
    cout << "Result: " << result << endl;
    assert(listnode_compare(result, expected));
  }

  return EXIT_SUCCESS;
}
