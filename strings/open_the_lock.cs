public class Solution {
    public int OpenLock(string[] deadends, string target) {
        HashSet<string> deadendsSet = [..deadends];
        if (deadendsSet.Contains(target)) return -1;
        
        string starter = "0000";
        if (deadendsSet.Contains(starter)) return -1;

        Span<int> moves = [-1, 1];
        Queue<(string, int)> q = [];
        q.Enqueue((starter, 0));

        HashSet<string> visited = [starter];

        try {
            while (q.TryDequeue(out var current)) {
                var (combination, actions) = current;
                if (combination == target) return actions;

                for (int i = 0; i < combination.Length; ++i) {
                    foreach (int move in moves) {
                        StringBuilder sb = new(combination);
                        sb[i] = (char) (((combination[i] - '0' + move + 10) % 10) + '0');
                        string newCombination = sb.ToString();

                        if (!deadendsSet.Contains(newCombination) && visited.Add(newCombination)) {
                            q.Enqueue((newCombination, actions + 1));
                        }
                    }
                }
            }

            return -1;
        } finally {
            GC.Collect(0);
        }
    }
}
