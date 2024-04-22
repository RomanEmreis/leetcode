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
    ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
        cin.tie(nullptr);
	    cout.tie(nullptr);
	    ios_base::sync_with_stdio(false);

        ListNode* curr = list1;

        for (int i = 0; i < a - 1; ++i) curr = curr->next;
        ListNode* tail1 = curr;

        for (int i = a - 1; i < b + 1; ++i) curr = curr->next;
        ListNode* tail2 = curr;

        tail1->next = list2;
        curr = list2->next;
        while (curr->next) curr = curr->next;
        curr->next = tail2;

        return list1;
    }
};
