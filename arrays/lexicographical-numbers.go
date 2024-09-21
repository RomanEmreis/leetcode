/*
  Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
  
  You must write an algorithm that runs in O(n) time and uses O(1) extra space. 
  
  Example 1:
  Input: n = 13
  Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
*/
func lexicalOrder(n int) []int {
    order := make([]int, 0);
    curr := 1;

    for i := 1; i <= n; i++ {
        order = append(order, curr);
        if curr * 10 <= n {
            curr *= 10;
        } else {
            if curr >= n {
                curr /= 10;
            }
            curr++;
            for curr % 10 == 0 {
                curr /= 10;
            }
        }
    }

    return order;
}
