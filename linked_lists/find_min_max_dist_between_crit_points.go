/*
A critical point in a linked list is defined as either a local maxima or a local minima.
A node is a local maxima if the current node has a value strictly greater than the previous node and the next node.
A node is a local minima if the current node has a value strictly smaller than the previous node and the next node.

Note that a node can only be a local maxima/minima if there exists both a previous node and a next node.

Given a linked list head, return an array of length 2 containing [minDistance, maxDistance] 
where minDistance is the minimum distance between any two distinct critical points and maxDistance is the maximum distance between any two distinct critical points.
If there are fewer than two critical points, return [-1, -1].

  Example 1:
    Input: head = [3,1]
    Output: [-1,-1]
    Explanation: There are no critical points in [3,1].

  Example 2:
    Input: head = [5,3,1,2,5,1,2]
    Output: [1,3]
    Explanation: There are three critical points:
    - [5,3,1,2,5,1,2]: The third node is a local minima because 1 is less than 3 and 2.
    - [5,3,1,2,5,1,2]: The fifth node is a local maxima because 5 is greater than 2 and 1.
    - [5,3,1,2,5,1,2]: The sixth node is a local minima because 1 is less than 5 and 2.
    The minimum distance is between the fifth and the sixth node. minDistance = 6 - 5 = 1.
    The maximum distance is between the third and the sixth node. maxDistance = 6 - 3 = 3.
*/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func nodesBetweenCriticalPoints(head *ListNode) []int {
    if head == nil || head.Next == nil { return []int{-1, -1}; }

    current, prev := head.Next, head;
    cp := make([]int, 0);
    i, diff := 2, int(^uint(0) >> 1);

    for current.Next != nil {
        if (current.Val > prev.Val && current.Val > current.Next.Val) || (current.Val < prev.Val && current.Val < current.Next.Val) {
            if len(cp) > 0 {
                if i - cp[len(cp) - 1] < diff {
                    diff = i - cp[len(cp)-1];
                }
            }
            cp = append(cp, i);
        }

        i++;
        prev = current;
        current = current.Next;
    }

    if len(cp) <= 1 {
        return []int{-1, -1};
    }

    return []int{diff, cp[len(cp)-1] - cp[0]};
}
