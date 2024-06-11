/*
  Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also in arr1.
  Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2. Elements that do not appear in arr2 should be placed at the end of arr1 in ascending order.

  Example:
    Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
    Output: [2,2,2,1,4,3,3,9,6,7,19]
*/
class Solution {
public:
    vector<int> relativeSortArray(vector<int>& arr1, vector<int>& arr2) {
        int m = *max_element(arr1.begin(), arr1.end());
        vector<int> count(m + 1);
        vector<int> result;

        for (const int& e : arr1) ++count[e];

        for (const int& e : arr2) {
            while (count[e] > 0) {
                result.push_back(e);
                --count[e];
            }
        }

        for (int i = 0; i <= m; ++i) {
            while (count[i] > 0) {
                result.push_back(i);
                --count[i];
            }
        }

        return result;
    }
};
