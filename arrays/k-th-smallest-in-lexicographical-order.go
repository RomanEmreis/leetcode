/*
  Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].
  
  Example 1:
    Input: n = 13, k = 2
    Output: 10
    Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.

  Example 2:
    Input: n = 1, k = 1
    Output: 1
*/
func findKthNumber(n int, k int) int {
    var curr int64 = 1;
    k--;

    for k > 0 {
        steps, first, last := int64(0), curr, curr;
        for first <= int64(n) {
            if last < int64(n) {
                steps += last - first + 1;
            } else {
                steps += int64(n) - first + 1;
            }
            first *= 10;
            last = last * 10 + 9;
        }
        if steps <= int64(k) {
            curr++;
            k -= int(steps);
        } else {
            curr *= 10;
            k--;
        }
    }

    return int(curr);
}
