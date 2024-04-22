class Solution {
public:
    string removeKdigits(string num, int k) {
        int n = num.length();
        if (n == k) return "0";

        vector<char> stack;
        
        for (const char& digit : num) {
            while (!stack.empty() && k > 0 && stack.back() > digit) {
                stack.pop_back();
                --k;
            }
            stack.push_back(digit);
        }

        while (k-- > 0) stack.pop_back();

        if (stack.empty()) return "0";

        string result = {};

        for (const char& c : stack) {
            if (c == '0' && result.empty()) continue;
            result += c;
        }

        return result.empty() ? "0" : result;
    }
};
