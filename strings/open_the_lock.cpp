class Solution {
public:
    int openLock(vector<string>& deadends, string target) {
        unordered_set<string> deadend(deadends.begin(), deadends.end());
        if (deadend.contains(target)) return -1;

        string starter = "0000";
        if (deadend.contains(starter)) return -1;

        vector<int> moves = {-1, 1};
        queue<pair<string, int>> q;
        q.push({starter, 0});

        unordered_set<string> visited;
        visited.insert(starter);

        while (!q.empty()) {
            auto [currentCombo, actions] = q.front();
            q.pop();

            if (currentCombo == target) return actions;

            for (int i = 0; i < 4; ++i) {
                for (int move : moves) {
                    int newDigit = (currentCombo[i] - '0' + move + 10) % 10;
                    string newCombo = currentCombo;
                    newCombo[i] = newDigit + '0';

                    if (!deadend.contains(newCombo) && !visited.contains(newCombo)) {
                        visited.insert(newCombo);
                        q.push({newCombo, actions + 1});
                    }
                }
            }
        }

        return -1;
    }
};
