/*
  Given a string n representing an integer, return the closest integer (not including itself), which is a palindrome. If there is a tie, return the smaller one.
  
  The closest is defined as the absolute difference minimized between two integers.
  
  Example 1:
    Input: n = "123"
    Output: "121"
  
  Example 2:
    Input: n = "1"
    Output: "0"
    Explanation: 0 and 2 are the closest palindromes but we return the smallest which is 0.
*/
class Solution {
public:
    string nearestPalindromic(string n) {
        long long original = stoll(n);
        int length = n.length();
        
        vector<long long> candidates;
        
        string half = n.substr(0, (length + 1) / 2);
        long long halfNum = stoll(half);
        
        for (int i = -1; i <= 1; ++i) {
            candidates.push_back(create_palindrome(to_string(halfNum + i), length));
        }
        
        candidates.push_back(static_cast<long long>(pow(10, length - 1)) - 1); 
        candidates.push_back(static_cast<long long>(pow(10, length)) + 1);    
        
        long long minDiff = LLONG_MAX;
        long long closest = LLONG_MAX;
        
        for (auto candidate : candidates) {
            if (candidate == original) continue;
            
            long long diff = abs(candidate - original);
            if (diff < minDiff || (diff == minDiff && candidate < closest)) {
                closest = candidate;
                minDiff = diff;
            }
        }
        
        return to_string(closest);
    }
private:
    long long create_palindrome(const string& half, int length) {
        string secondHalf = half;
        reverse(secondHalf.begin(), secondHalf.end());
    
        if (length % 2 == 0) return stoll(half + secondHalf);
        else return stoll(half + secondHalf.substr(1));
    }
};
