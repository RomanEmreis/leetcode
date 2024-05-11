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
public class Solution {
    public double MincostToHireWorkers(int[] quality, int[] wage, int k) {
        double result = double.MaxValue;
        double currentQuality = 0.0d;

        Span<(int, double)> heap = new (int, double)[wage.Length];
        for (int i = 0; i < wage.Length; ++i) {
            heap[i] = (quality[i], (double) wage[i] / quality[i]);
        }

        heap.Sort(new Comparer());

        PriorityQueue<int, int> hq = new(k + 1);

        for (int i = 0; i < heap.Length; ++i) {
            var (q, r) = heap[i];

            currentQuality += q;
            hq.Enqueue(q, -q);

            if (hq.Count > k) currentQuality -= hq.Dequeue();
            if (hq.Count == k) result = Math.Min(result, currentQuality * r);
        }

        return result;
    }

    private class Comparer : IComparer<(int, double)> {
        public int Compare((int, double) x, (int, double) y) => x.Item2.CompareTo(y.Item2);
    }
}
