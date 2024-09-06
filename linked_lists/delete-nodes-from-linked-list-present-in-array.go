/*
  You are given an array of integers nums and the head of a linked list. Return the head of the modified linked list after removing all nodes from the linked list that have a value that exists in nums.
   
  Example 1:
    Input: nums = [1,2,3], head = [1,2,3,4,5]
    Output: [4,5]
*/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func modifiedList(nums []int, head *ListNode) *ListNode {
    toRemove := make(map[int]bool);
    for _, num := range nums {
        toRemove[num] = true;
    }

    guard := &ListNode{0,head};
    current := guard;

    for current != nil && current.Next != nil {
        if _, ok := toRemove[current.Next.Val]; ok {
            current.Next = current.Next.Next;
        } else {
            current = current.Next;
        }
    }

    return guard.Next;
}
