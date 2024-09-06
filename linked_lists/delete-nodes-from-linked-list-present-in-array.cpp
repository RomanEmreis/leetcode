/*
  You are given an array of integers nums and the head of a linked list. Return the head of the modified linked list after removing all nodes from the linked list that have a value that exists in nums.
   
  Example 1:
    Input: nums = [1,2,3], head = [1,2,3,4,5]
    Output: [4,5]
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
    ListNode* modifiedList(vector<int>& nums, ListNode* head) {
        unordered_set<int> to_remove(nums.begin(), nums.end());

        ListNode* guard = new ListNode(0, head);
        ListNode* current = guard;

        while (current && current->next) {
            if (to_remove.contains(current->next->val)) {
                current->next = current->next->next;
            } else {
                current = current->next;
            }
        }

        return guard->next;
    }
};
