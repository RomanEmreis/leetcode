/*
  In the video game Fallout 4, the quest "Road to Freedom" requires players to reach a metal dial called the "Freedom Trail Ring" and use the dial to spell a specific keyword to open the door.

  Given a string ring that represents the code engraved on the outer ring and another string key that represents the keyword that needs to be spelled, return the minimum number of steps to spell all the characters in the keyword.

  Initially, the first character of the ring is aligned at the "12:00" direction. You should spell all the characters in key one by one by rotating ring clockwise or anticlockwise to make each character of the string key aligned at the "12:00" direction
  and then by pressing the center button.

  At the stage of rotating the ring to spell the key character key[i]:

  You can rotate the ring clockwise or anticlockwise by one place, which counts as one step. The final purpose of the rotation is to align one of ring's characters at the "12:00" direction, where this character must equal key[i].
  If the character key[i] has been aligned at the "12:00" direction, press the center button to spell, which also counts as one step. After the pressing, you could begin to spell the next character in the key (next stage). 
  Otherwise, you have finished all the spelling.
*/
class Solution {
private:
    int dist(int size, int p, int t){
        return min(abs(t - p), size - abs(t - p));
    }

public:
    int findRotateSteps(string& ring, string& key){
        int m[26][100] = {{0}};
        int cnt[26] = {0};
        int dp[100][100] = {{0}};

        const int rn = ring.size();
        const int kn = key.size();

        for (int i = 0; i < rn; ++i) {
            m[ring[i]-'a'][cnt[ring[i]-'a']++] = i;
        }

        for (int i = 0; i < cnt[key[0]-'a']; ++i) {
            dp[0][m[key[0]-'a'][i]] = dist(rn, 0, m[key[0]-'a'][i]) + 1;
        }

        for (int i = 1; i < kn; ++i) {
            for (int j = 0; j < cnt[key[i]-'a']; ++j) {
                int mini = INT_MAX;
                for (int k = 0; k < cnt[key[i-1]-'a']; ++k) {
                    mini = min(mini, dp[i-1][m[key[i-1]-'a'][k]] + dist(rn, m[key[i]-'a'][j], m[key[i-1]-'a'][k]) + 1);
                }
                dp[i][m[key[i]-'a'][j]] = mini;
            }
        }

        int res = INT_MAX;

        for (int i = 0; i < cnt[key.back()-'a']; ++i) {
            res = min(res, dp[kn-1][m[key[kn-1]-'a'][i]]);
        }

        return res;
    }
};
