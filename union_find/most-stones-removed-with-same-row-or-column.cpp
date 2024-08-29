/*
  On a 2D plane, we place n stones at some integer coordinate points. Each coordinate point may have at most one stone.
  A stone can be removed if it shares either the same row or the same column as another stone that has not been removed.
  
  Given an array stones of length n where stones[i] = [xi, yi] represents the location of the ith stone, return the largest possible number of stones that can be removed.
  
  Example 1:
    Input: stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
    Output: 5
    Explanation: One way to remove 5 stones is as follows:
    1. Remove stone [2,2] because it shares the same row as [2,1].
    2. Remove stone [2,1] because it shares the same column as [0,1].
    3. Remove stone [1,2] because it shares the same row as [1,0].
    4. Remove stone [1,0] because it shares the same column as [0,0].
    5. Remove stone [0,1] because it shares the same row as [0,0].
    Stone [0,0] cannot be removed since it does not share a row/column with another stone still on the plane.

  Example 2:
    Input: stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
    Output: 3
    Explanation: One way to make 3 moves is as follows:
    1. Remove stone [2,2] because it shares the same row as [2,0].
    2. Remove stone [2,0] because it shares the same column as [0,0].
    3. Remove stone [0,2] because it shares the same row as [0,0].
    Stones [0,0] and [1,1] cannot be removed since they do not share a row/column with another stone still on the plane.
  
  Example 3:
    Input: stones = [[0,0]]
    Output: 0
    Explanation: [0,0] is the only stone on the plane, so you cannot remove it.
*/
struct union_find {
    unordered_map<int, int> parent;
    int find(int x);
    void union_(int x, int y);
};

int union_find::find(int x) {
    if (!parent.contains(x)) parent[x] = x;
    if (parent[x] != x) parent[x] = find(parent[x]);
    return parent[x];
}

void union_find::union_(int x, int y) {
    int root_x = find(x);
    int root_y = find(y);
    if (root_x != root_y) parent[root_x] = root_y;
}

class Solution {
public:
    int removeStones(vector<vector<int>>& stones) {
        union_find uf;
        for (vector<int>& stone : stones) {
            uf.union_(stone[0], ~stone[1]);
        }

        unordered_set<int> seen;
        for (vector<int>& stone : stones) {
            seen.insert(uf.find(stone[0]));
        }

        return stones.size() - seen.size();
    }
};
