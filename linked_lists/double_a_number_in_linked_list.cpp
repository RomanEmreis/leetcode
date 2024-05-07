/*
  You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.
  Return the head of the linked list after doubling it.

  Input: head = [1,8,9]
  Output: [3,7,8]
  Explanation: The figure above corresponds to the given linked list which represents the number 189. Hence, the returned linked list represents the number 189 * 2 = 378.
*/
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
private:
    int doubleVal(ListNode* head) {
        if (!head) return 0;

        int carry = doubleVal(head->next);

        int doubled_val = head->val * 2;
        int reminder = doubled_val % 10;

        head->val = reminder + carry;

        return (doubled_val - reminder) / 10;
    }

public:
    ListNode* doubleIt(ListNode* head) {
        int carry = doubleVal(head);
        if (carry != 0) {
            return new ListNode(carry, head);
        }

        return head;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
