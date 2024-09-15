/*
  You are given an array arr of positive integers. You are also given the array queries where queries[i] = [lefti, righti].
  For each query i compute the XOR of elements from lefti to righti (that is, arr[lefti] XOR arr[lefti + 1] XOR ... XOR arr[righti] ).
  
  Return an array answer where answer[i] is the answer to the ith query.

  Example 1:
    Input: arr = [1,3,4,8], queries = [[0,1],[1,2],[0,3],[3,3]]
    Output: [2,7,14,8] 
    Explanation: 
    The binary representation of the elements in the array are:
    1 = 0001 
    3 = 0011 
    4 = 0100 
    8 = 1000 
    The XOR values for queries are:
    [0,1] = 1 xor 3 = 2 
    [1,2] = 3 xor 4 = 7 
    [0,3] = 1 xor 3 xor 4 xor 8 = 14 
    [3,3] = 8
*/
class Solution {
public:
    vector<int> xorQueries(vector<int>& arr, vector<vector<int>>& queries) {
        for (size_t i = 1; i < arr.size(); ++i) {
            arr[i] ^= arr[i - 1];
        }

        vector<int> result(queries.size());
        for (size_t i = 0; i < queries.size(); ++i) {
            int l = queries[i][0], r = queries[i][1];
            result[i] = l > 0 ? arr[r] ^ arr[l - 1] : arr[r];
        }

        return result;
    }
};