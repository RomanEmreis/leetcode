/*
  You have n jobs and m workers. You are given three arrays: difficulty, profit, and worker where:
  difficulty[i] and profit[i] are the difficulty and the profit of the ith job, and
  worker[j] is the ability of jth worker (i.e., the jth worker can only complete a job with difficulty at most worker[j]).
  Every worker can be assigned at most one job, but one job can be completed multiple times.

  For example, if three workers attempt the same job that pays $1, then the total profit will be $3. If a worker cannot complete any job, their profit is $0.
  Return the maximum profit we can achieve after assigning the workers to the jobs.

  Example:
    Input: difficulty = [2,4,6,8,10], profit = [10,20,30,40,50], worker = [4,5,6,7]
    Output: 100
    Explanation: Workers are assigned jobs of difficulty [4,4,6,6] and they get a profit of [20,20,30,30] separately.
*/
class Solution {
public:
    int maxProfitAssignment(vector<int>& difficulty, vector<int>& profit, vector<int>& worker) {
        int res = 0, j = 0, best = 0, n = worker.size();
        vector<pair<int, int>> jobs(n);
        for(int i = 0; i < n; ++i) {
            jobs[i] = {difficulty[i], profit[i]};
        }

        sort(begin(jobs), end(jobs));
        sort(begin(worker), end(worker));

        for(const int& work : worker) {
            while(j < n && work >= jobs[j].first) best = max(best, jobs[j++].second);
            res += best;
        }

        return res;
    }
};

auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
