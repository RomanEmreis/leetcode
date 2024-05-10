/*
  You are given a sorted integer array arr containing 1 and prime numbers, where all the integers of arr are unique. You are also given an integer k.
  For every i and j where 0 <= i < j < arr.length, we consider the fraction arr[i] / arr[j].
  Return the kth smallest fraction considered. Return your answer as an array of integers of size 2, where answer[0] == arr[i] and answer[1] == arr[j].

  Example 1:
    Input: arr = [1,2,3,5], k = 3
    Output: [2,5]
    Explanation: The fractions to be considered in sorted order are:
    1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
    The third fraction is 2/5.
*/
struct Compare {
    bool operator()(pair<double, pair<int, int>> a, pair<double, pair<int, int>> b) {
        return a.first > b.first;
    }
};

class Solution {
public:
    vector<int> kthSmallestPrimeFraction(vector<int>& arr, int k) {
        int n = arr.size();
        if (n == 2) return arr;

        // We use priority queue to hold {fraction value, {numerator index, denominator index}}
        priority_queue<pair<double, pair<int, int>>, vector<pair<double, pair<int, int>>>, Compare> pq;

        // Initially we push fraction values with denominator as last array element and numerators as all other elements
        for(int i=0; i < n - 1; i++)
            pq.push({(double) arr[i] / arr[n-1], {i, n - 1}});
    
        // Now we pop top fraction k-1 times and keep pushing new fractions into the queue
        for(int count=0; count<k-1; count++) {
            // Get the top fraction
            pair<double, pair<int, int>> top = pq.top(); pq.pop();

            // If the numerator of the fraction is not the last element
            // Update the denominator to next smaller value and push the new fraction to queue
            if(top.second.second - 1 > top.second.first)
                pq.push({(double)arr[top.second.first] / arr[top.second.second - 1], {top.second.first, top.second.second - 1}});
        }

        // Return the k-th smallest fraction
        return {arr[pq.top().second.first], arr[pq.top().second.second]};
    }
};

auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
