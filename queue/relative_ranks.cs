/*
  You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.
  The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:

  The 1st place athlete's rank is "Gold Medal".
  The 2nd place athlete's rank is "Silver Medal".
  The 3rd place athlete's rank is "Bronze Medal".
  For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
  Return an array answer of size n where answer[i] is the rank of the ith athlete.

  Example:
    Input: score = [5,4,3,2,1]
    Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
    Explanation: The placements are [1st, 2nd, 3rd, 4th, 5th].
*/
public class Solution {
    private const string 
        Gold = "Gold Medal",
        Silver = "Silver Medal",
        Bronze = "Bronze Medal";

    public string[] FindRelativeRanks(int[] score) {
        if (score.Length == 1) return [Gold];

        queue.Clear();
        for (int i = 0; i < score.Length; ++i) queue.Enqueue(i, score[i]);

        string[] result = new string[score.Length];

        int place = 0;
        while (queue.TryDequeue(out int index, out int _)) {
            result[index] = ++place switch 
            {
                1 => Gold,
                2 => Silver,
                3 => Bronze,
                _ => place.ToString()
            };
        }

        return result;
    }

    private static readonly IComparer<int> comparer = Comparer<int>.Create(CompareScore);
    private static readonly PriorityQueue<int, int> queue = new(comparer);
    private static int CompareScore(int x, int y) => -x.CompareTo(y);
}
