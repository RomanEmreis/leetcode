#include <iostream>
#include <vector>
#include <set>

using namespace std;

long long countSubarrays(vector<int>& nums, int k) {
    int l = 0, r = 0, n = nums.size();
    long long result = 0;
    int mx = *max_element(begin(nums), end(nums));
    while (r < n) {
        if (nums[r++] == mx) --k;
        while (k == 0) if (nums[l++] == mx) ++k;
        result += l;
    }
    return result;
}

int main()
{
    vector<int> y{1,3,2,3,3};
    long long x = countSubarrays(y, 2);
}
