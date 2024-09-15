/*
  Given the string s, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
  
  Example 1:
    Input: s = "eleetminicoworoep"
    Output: 13
    Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
*/
class Solution {
public:
    int findTheLongestSubstring(string s) {
        int result = 0, mask = 0;
        vector<int> seen(1 << 5, -1);
        seen[0] = 0;

        for (size_t i = 0; i < s.length(); ++i) {
            switch (s[i]) {
                case 'a':
                    mask ^= 1 << 0;
                    break;
                case 'e':
                    mask ^= 1 << 1;
                    break;
                case 'i':
                    mask ^= 1 << 2;
                    break;
                case 'o':
                    mask ^= 1 << 3;
                    break;
                case 'u':
                    mask ^= 1 << 4;
                    break;
            }

            if (seen[mask] >= 0) {
                result = max(result, int(i + 1 - seen[mask]));
            } else {
                seen[mask] = i + 1;
            }
        }

        return result;
    }
};
