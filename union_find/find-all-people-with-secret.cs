/*
  2092. Find All People With Secret
  
  You are given an integer n indicating there are n people numbered from 0 to n - 1. You are also given a 0-indexed 2D integer array meetings where meetings[i] = [xi, yi, timei]
  indicates that person xi and person yi have a meeting at timei. A person may attend multiple meetings at the same time. Finally, you are given an integer firstPerson.
  Person 0 has a secret and initially shares the secret with a person firstPerson at time 0. This secret is then shared every time a meeting takes place with a person that has the secret. 
  More formally, for every meeting, if a person xi has the secret at timei, then they will share the secret with person yi, and vice versa.
  
  The secrets are shared instantaneously. That is, a person may receive the secret and share it with people in other meetings within the same time frame.
  
  Return a list of all the people that have the secret after all the meetings have taken place. You may return the answer in any order.
  
  Example 1:
  Input: n = 6, meetings = [[1,2,5],[2,3,8],[1,5,10]], firstPerson = 1
  Output: [0,1,2,3,5]
  Explanation:
  At time 0, person 0 shares the secret with person 1.
  At time 5, person 1 shares the secret with person 2.
  At time 8, person 2 shares the secret with person 3.
  At time 10, person 1 shares the secret with person 5.​​​​
  Thus, people 0, 1, 2, 3, and 5 know the secret after all the meetings.
  
  Example 2:
  Input: n = 4, meetings = [[3,1,3],[1,2,2],[0,3,3]], firstPerson = 3
  Output: [0,1,3]
  Explanation:
  At time 0, person 0 shares the secret with person 3.
  At time 2, neither person 1 nor person 2 know the secret.
  At time 3, person 3 shares the secret with person 0 and person 1.
  Thus, people 0, 1, and 3 know the secret after all the meetings.
  
  Example 3:
  Input: n = 5, meetings = [[3,4,2],[1,2,1],[2,3,1]], firstPerson = 1
  Output: [0,1,2,3,4]
  Explanation:
  At time 0, person 0 shares the secret with person 1.
  At time 1, person 1 shares the secret with person 2, and person 2 shares the secret with person 3.
  Note that person 2 can share the secret at the same time as receiving it.
  At time 2, person 3 shares the secret with person 4.
  Thus, people 0, 1, 2, 3, and 4 know the secret after all the meetings.
*/
public class Solution {
    public IList<int> FindAllPeople(int n, int[][] meetings, int firstPerson) {
        List<int> res = [];
        UnionFind uf = new(n);
        SortedDictionary<int, List<(int, int)>> timeToPairs = [];

        uf.Union(0, firstPerson);

        foreach (int[] m in meetings) {
            int x = m[0];
            int y = m[1];
            int t = m[2];
            if (timeToPairs.TryGetValue(t, out var pairs))
                pairs.Add((x, y));
            else 
                timeToPairs[t] = [(x, y)];
        }

        foreach (var (_, pairs) in timeToPairs) {
            HashSet<int> people = [];
            foreach (var (x, y) in pairs) {
                uf.Union(x, y);
                people.Add(x);
                people.Add(y);
            }

            foreach (int p in people)
                if (!uf.IsConnected(p, 0))
                    uf.Reset(p);
        }

        for (int i = 0; i < n; ++i)
            if (uf.IsConnected(i, 0))
                res.Add(i);

        return res;
    }
}

public readonly ref struct UnionFind {
    private readonly int[] id;
    private readonly int[] rank;

    public UnionFind(int n) {
        id = new int[n];
        rank = new int[n];

        for (int i = 0; i < id.Length; ++i)
            id[i] = i;
    }

    public void Union(int u, int v) {
        int i = Find(u);
        int j = Find(v);

        if (i == j)
            return;

        if (rank[i] < rank[j])
            id[i] = j;
        else if (rank[i]> rank[j])
            id[j] = i;
        else {
            id[i] = j;
            ++rank[j];
        }
    }

    public bool IsConnected(int u, int v) => Find(u) == Find(v);

    public void Reset(int u) => id[u] = u;

    private int Find(int u) => id[u] == u ? u : (id[u] = Find(id[u]));
}
