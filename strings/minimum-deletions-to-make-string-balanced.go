/*
  You are given a string s consisting only of characters 'a' and 'b'​​​​.
  You can delete any number of characters in s to make s balanced. s is balanced if there is no pair of indices (i,j) such that i < j and s[i] = 'b' and s[j]= 'a'.
  Return the minimum number of deletions needed to make s balanced.

  Example:
    Input: s = "aababbab"
    Output: 2
    Explanation: You can either:
    Delete the characters at 0-indexed positions 2 and 6 ("aababbab" -> "aaabbb"), or
    Delete the characters at 0-indexed positions 3 and 6 ("aababbab" -> "aabbbb").
*/
func minimumDeletions(s string) int {
    result, b := 0, 0;

    for _, ch := range s {
        if ch == 'b' {
            b++;
        } else if 1 + result > b {
            result = b;
        } else {
            result++;
        }
    }

    return result;
}
