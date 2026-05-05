#include <vector>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* listnode_create(vector<int> values) {
  ListNode temp;
  ListNode* tail = &temp;

  for (int val : values) {
    tail->next = new ListNode(val);
    tail = tail->next;
  }
  ListNode* head = temp.next;
  return head;
}

bool listnode_compare(ListNode* a, ListNode* b) {
  while (a && b) {
    if (a->val != b->val)
      return false;
    a = a->next;
    b = b->next;
  }

  return a == nullptr && b == nullptr;
}
