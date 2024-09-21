/*
  Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
  
  You must write an algorithm that runs in O(n) time and uses O(1) extra space. 
  
  Example 1:
  Input: n = 13
  Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
*/
public class Solution {
    public IList<int> LexicalOrder(int n) {
        IList<int> order = new List<int>();
        int cur = 1;
        for(int i = 1; i <= n; ++i) {
            order.Add(cur);
            if(cur * 10 <= n)  cur *= 10;
            else {
                if(cur >= n) cur /= 10;
                ++cur;
                while(cur % 10 == 0) cur /= 10;
            }
        }

        return order;
    }
}
