class Solution {
public:
    int countStudents(vector<int>& students, vector<int>& sandwiches) {
        if (students.size() == 1) return students[0] == sandwiches[0] ? 0 : 1;

        queue<int> q;
        stack<int> s;

        for (int i = 0; i < students.size(); ++i) q.push(students[i]);
        for (int i = sandwiches.size() - 1; i >=0; --i) s.push(sandwiches[i]);

        int refused = 0;
        while (!q.empty() && !s.empty()) {
            int student = q.front();
            q.pop();
            if (student == s.top()) {
                s.pop();
                refused = 0;
            } else {
                q.push(student);
                ++refused;
            }

            if (refused > students.size()) return q.size();
        }

        return q.size();
    }
};
