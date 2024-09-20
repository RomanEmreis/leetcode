/*
  You are given a string s. You can convert s to a palindrome by adding characters in front of it.
  
  Return the shortest palindrome you can find by performing this transformation.
  
  Example 1:
  Input: s = "aacecaaa"
  Output: "aaacecaaa"
  
  Example 2:
  Input: s = "abcd"
  Output: "dcbabcd"
*/
func shortestPalindrome(s string) string {
    left := 0;
    for right := len(s) - 1; right >= 0; right-- {
        if s[left] == s[right] {
            left++;
        }
    }
    if left == len(s) {
        return s;
    }

    suffix := s[left:];
    prefix := reverse(suffix);
    mid := shortestPalindrome(s[0:left]);

    return prefix + mid + suffix;
}

func reverse(s string) string {
	var sb strings.Builder
	for i := len(s) - 1; i >= 0; i-- {
		sb.WriteByte(s[i])
	}
	return sb.String()
}
