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
public class Solution {
    private readonly Dictionary<string, int> memo = [];

    public int FindRotateSteps(string ring, string key) {
        return Solve(key, ring, 0);
    }

    private int Solve(string key, string ring, int i) {
        if (i == key.Length) return 0;

        string keyRing = ring + i.ToString();
        if (memo.ContainsKey(keyRing)) return memo[keyRing];

        char c = key[i];
        int steps = int.MaxValue;

        for (int j = 0; j < ring.Length; ++j) {
            if (ring[j] == c) {
                int rotations = Math.Min(j, ring.Length - j);
                string rotated = Rotate(ring, ref j);

                steps = Math.Min(steps, 1 + rotations + Solve(key, rotated, i + 1));
            }
        }

        return memo[keyRing] = steps;
    }

    private static string Rotate(string ring, ref int steps) {
        return ring[steps..] + ring[0..steps];
    }
}
