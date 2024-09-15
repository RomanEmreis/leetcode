/*
  Given the string s, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
  
  Example 1:
    Input: s = "eleetminicoworoep"
    Output: 13
    Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
*/
func findTheLongestSubstring(s string) int {
	vowelIndex := map[rune]int{
		'a': 0,
		'e': 1,
		'i': 2,
		'o': 3,
		'u': 4,
	}

    firstOccurence := make(map[int]int);
    firstOccurence[0] = -1;

    result, mask := 0, 0;

    for i, ch := range s {
        if bit, found := vowelIndex[ch]; found {
            mask ^= (1 << bit);
        }

        if val, exists := firstOccurence[mask]; exists {
            if i - val > result {
                result = i - val;
            }
        }else {
            firstOccurence[mask] = i;
        }
    }

    return result;
}
