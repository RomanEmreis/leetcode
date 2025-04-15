/*
  2179. Count Good Triplets in an Array
  
  You are given two 0-indexed arrays nums1 and nums2 of length n, both of which are permutations of [0, 1, ..., n - 1].
  
  A good triplet is a set of 3 distinct values which are present in increasing order by position both in nums1 and nums2. In other words, 
  if we consider pos1v as the index of the value v in nums1 and pos2v as the index of the value v in nums2, then a good triplet will be a set (x, y, z) where 0 <= x, y, z <= n - 1,
  such that pos1x < pos1y < pos1z and pos2x < pos2y < pos2z.
  
  Return the total number of good triplets.
  
  Example 1:
  Input: nums1 = [2,0,1,3], nums2 = [0,1,2,3]
  Output: 1
  Explanation: 
  There are 4 triplets (x,y,z) such that pos1x < pos1y < pos1z. They are (2,0,1), (2,0,3), (2,1,3), and (0,1,3). 
  Out of those triplets, only the triplet (0,1,3) satisfies pos2x < pos2y < pos2z. Hence, there is only 1 good triplet.
  
  Example 2:
  Input: nums1 = [4,0,1,3,2], nums2 = [4,1,0,2,3]
  Output: 4
  Explanation: The 4 good triplets are (4,0,3), (4,0,2), (4,1,3), and (4,1,2).
*/
public class Solution {
    public long GoodTriplets(int[] nums1, int[] nums2) {
        int n = nums1.Length;
        Span<int> pos = stackalloc int[n];
        for (int i = 0; i < nums2.Length; ++i) {
            pos[nums2[i]] = i + 1;
        }

        Tree t = new(n);
        long result = 0;

        foreach (int num in nums1) {
            int p = pos[num];
            int l = t.Query(p);
            int r = n - p - (t.Query(n) - t.Query(p));
            result += 1L * l * r;

            t.Update(p, 1);
        }

        return result;
    }
}

unsafe ref struct Tree(int n) {
    private Span<int> _count = stackalloc int[n + 1];
    private int _length = n;
    
    private static int LowBit(int x) => x & -x;

    public void Update(int x, int delta) {
        while (x <= _length) {
            _count[x] += delta;
            x += LowBit(x);
        }
    }

    public int Query(int x) {
        int res = 0;
        while (x > 0) {
            res += _count[x];
            x -= LowBit(x);
        }
        return res;
    }
}
