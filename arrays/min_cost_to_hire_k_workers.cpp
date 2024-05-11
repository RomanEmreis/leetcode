/*
  There are n workers. You are given two integer arrays quality and wage where quality[i] is the quality of the ith worker and wage[i] is the minimum wage expectation for the ith worker.
  We want to hire exactly k workers to form a paid group. To hire a group of k workers, we must pay them according to the following rules:

  Every worker in the paid group must be paid at least their minimum wage expectation.
  * In the group, each worker's pay must be directly proportional to their quality. This means if a worker’s quality is double that of another worker in the group, then they must be paid twice as much as the other worker.
  * Given the integer k, return the least amount of money needed to form a paid group satisfying the above conditions. Answers within 10-5 of the actual answer will be accepted.

  Example 1:
    Input: quality = [10,20,5], wage = [70,50,30], k = 2
    Output: 105.00000
    Explanation: We pay 70 to 0th worker and 35 to 2nd worker.
*/
class Solution {
public:
    double mincostToHireWorkers(vector<int>& quality, vector<int>& wage, int k) {
        int n = quality.size();
        double result = DBL_MAX, total = 0.0;

        vector<pair<double, int>> heap;
        for (int i = 0; i < n; ++i) {
            heap.push_back({static_cast<double>(wage[i]) / quality[i], quality[i]});
        }

        sort(heap.begin(), heap.end());

        priority_queue<int> pq;

        for (int i = 0; i < n; ++i) {
            auto [r, q] = heap[i];

            total += q;
            pq.push(q);

            if (pq.size() > k) {
                total -= pq.top();
                pq.pop();
            }

            if (pq.size() == k) {
                result = min(result, total * r);
            }
        }

        return result;
    }
};

auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
