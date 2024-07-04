/*
  You are given the head of a linked list, which contains a series of integers separated by 0's. The beginning and end of the linked list will have Node.val == 0.
  For every two consecutive 0's, merge all the nodes lying in between them into a single node whose value is the sum of all the merged nodes. The modified list should not contain any 0's.

  Return the head of the modified linked list.

  Example:
    Input: head = [0,3,1,0,4,5,2,0]
    Output: [4,11]
    Explanation: 
    The above figure represents the given linked list. The modified list contains
    - The sum of the nodes marked in green: 3 + 1 = 4.
    - The sum of the nodes marked in red: 4 + 5 + 2 = 11.
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
public:
    ListNode* mergeNodes(ListNode* head) {
        ListNode* current = head->next;
        ListNode* tail = head;

        while (current) {
            tail->val += current->val;
            current = current->next;

            if (!current) break;

            if (current->val == 0) {
                tail->next = current->next ? current : nullptr;
                tail = current;
                current = current->next;
            }
        }

        return head;
    }
};
