/*
  A wonderful string is a string where at most one letter appears an odd number of times.

  For example, "ccjjc" and "abab" are wonderful, but "ab" is not.
  Given a string word that consists of the first ten lowercase English letters ('a' through 'j'), return the number of wonderful non-empty substrings in word. If the same substring appears multiple times in word, then count each occurrence separately.

  A substring is a contiguous sequence of characters in a string.

  Solution:
    * Initialize result (res) to 0. This will contain the final count of wonderful strings.
    * Initialize array count[] of size 2^10 to store the frequency of each mask while traversing the string.
    * Traverse the given string and at each character calculate the cumulative mask. Check if the current mask or any previous mask with only one bit different from the current mask is wonderful or not, if yes then add it to the result.
    * Finally, return the count of all wonderful substrings as result.
*/
public class Solution {
    public long WonderfulSubstrings(string word) {
	    long result = 0;
	    int mask = 0;

        Span<long> count = stackalloc long[1024];
	    count[0] = 1;

	    for (int i = 0; i < word.Length; ++i) {
	    	mask ^= 1 << (word[i] - 'a');
	    	result += count[mask];

	    	for (int j = 0; j < 10; ++j)
	    		result += count[mask ^ (1 << j)];

	    	++count[mask];
	    }

	    return result;
    }
}
