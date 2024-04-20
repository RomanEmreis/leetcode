public class Solution {
    public string LongestCommonPrefix(string[] strs) {
        if (strs.Length == 0) {
            return string.Empty;
        }

        Array.Sort(strs, (c,n) => c.Length.CompareTo(n.Length));
        ReadOnlySpan<char> p = strs[0].AsSpan();

        for (int i = 1; i < strs.Length; i++) {
            while(p.Length > 0 && !strs[i].AsSpan().StartsWith(p)) {
                p = p.Slice(0, p.Length - 1);
            }
            
            if (p.Length == 0) {
                return string.Empty;
            }
        }

        return p.ToString();
    }
}
