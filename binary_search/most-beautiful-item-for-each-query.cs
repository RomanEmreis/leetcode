/*
  You are given a 2D integer array items where items[i] = [pricei, beautyi] denotes the price and beauty of an item respectively.
  You are also given a 0-indexed integer array queries. For each queries[j], you want to determine the maximum beauty of an item whose price is less than or equal to queries[j]. If no such item exists, then the answer to this query is 0.
  
  Return an array answer of the same length as queries where answer[j] is the answer to the jth query.

  Example 1:
    Input: items = [[1,2],[3,2],[2,4],[5,6],[3,5]], queries = [1,2,3,4,5,6]
    Output: [2,4,5,5,6,6]
    Explanation:
    - For queries[0]=1, [1,2] is the only item which has price <= 1. Hence, the answer for this query is 2.
    - For queries[1]=2, the items which can be considered are [1,2] and [2,4]. 
      The maximum beauty among them is 4.
    - For queries[2]=3 and queries[3]=4, the items which can be considered are [1,2], [3,2], [2,4], and [3,5].
      The maximum beauty among them is 5.
    - For queries[4]=5 and queries[5]=6, all items can be considered.
      Hence, the answer for them is the maximum beauty of all items, i.e., 6.

  Example 2:
    Input: items = [[1,2],[1,2],[1,3],[1,4]], queries = [1]
    Output: [4]
    Explanation: 
    The price of every item is equal to 1, so we choose the item with the maximum beauty 4. 
    Note that multiple items can have the same price and/or beauty. 
*/
public class Solution {
    public int[] MaximumBeauty(int[][] items, int[] queries) {
        Array.Sort(items, (x, y) => x[0] - y[0]);

        for (int i = 1; i < items.Length; ++i) {
            items[i][1] = Math.Max(items[i][1], items[i - 1][1]);
        }

        int n = items.Length;
        int[] result = new int[queries.Length];
        for (int i = 0; i < queries.Length; ++i) {
            int l = 0, r = n;
            while (l < r) {
                int m = (l + r) >> 1;
                if (queries[i] < items[m][0]) r = m;
                else l = m + 1;
            }
            result[i] = --l >= 0 ? items[l][1] : 0;
        }

        return result;

    }
}
