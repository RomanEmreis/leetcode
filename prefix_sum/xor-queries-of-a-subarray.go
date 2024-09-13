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
func xorQueries(arr []int, queries [][]int) []int {
    prefix := make([]int, len(arr) + 1);
    for i := 1; i <= len(arr); i++ {
        prefix[i] = prefix[i - 1] ^ arr[i - 1];
    }

    result := make([]int, len(queries));
    for i := 0; i < len(queries); i++ {
        result[i] = prefix[queries[i][0]] ^ prefix[queries[i][1] + 1];
    }

    return result;
}
