/*
  Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also in arr1.
  Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2. Elements that do not appear in arr2 should be placed at the end of arr1 in ascending order.

  Example:
    Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
    Output: [2,2,2,1,4,3,3,9,6,7,19]
*/
public class Solution {
    public int[] RelativeSortArray(int[] arr1, int[] arr2) {
        Dictionary<int, int> map = [];

        for (int i = 0; i < arr2.Length; ++i) {
            map[arr2[i]] = i;
        }

        Array.Sort(arr1, Comparer);

        return arr1;

        int Comparer(int x, int y) {
            if (map.TryGetValue(x, out int p1) && map.TryGetValue(y, out int p2)) {
                return p1.CompareTo(p2);
            } else if (map.ContainsKey(x)) {
                return -1;
            } else if (map.ContainsKey(y)) {
                return 1;
            } else {
                return x.CompareTo(y);
            }
        }
    }
}
