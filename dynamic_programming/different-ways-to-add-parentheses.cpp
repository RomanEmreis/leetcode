/*
  Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.
  The test cases are generated such that the output values fit in a 32-bit integer and the number of different results does not exceed 104.
  
  Example 1:
    Input: expression = "2-1-1"
    Output: [0,2]
    Explanation:
    ((2-1)-1) = 0 
    (2-(1-1)) = 2
*/
class Solution {
public:
    vector<int> diffWaysToCompute(string expression) {
        if (is_only_digits(expression)) {
            return {stoi(expression)};
        }

        vector<int> results;
        int n = expression.length();

        for (size_t i = 0; i < n; ++i) {
            char c = expression[i];
            if (c == '+' || c == '-' || c == '*') {
                vector<int> left_results = diffWaysToCompute(expression.substr(0, i));
                vector<int> right_results = diffWaysToCompute(expression.substr(i + 1, n));

                for (const int ll : left_results) {
                    for (const int rr : right_results) {
                        int result = 0;
                        switch (c) {
                            case '+':
                                result = ll + rr;
                                break;
                            case '-':
                                result = ll - rr;
                                break;
                            case '*':
                                result = ll * rr;
                                break;
                        }
                        results.push_back(result);
                    }
                }
            }
        }

        return results;
    }
private:
    static bool is_only_digits(const string& expression) {
        for (const char c : expression) {
            if (!isdigit(c)) return false;
        }
        return true;
    }
};
