/*
  Given a string expression representing an expression of fraction addition and subtraction, return the calculation result in string format.
  
  The final result should be an irreducible fraction. If your final result is an integer, change it to the format of a fraction that has a denominator 1. So in this case, 2 should be converted to 2/1.
  
  Example 1:
    Input: expression = "-1/2+1/2"
    Output: "0/1"
  
  Example 2:
    Input: expression = "-1/2+1/2+1/3"
    Output: "1/3"
  
  Example 3:
    Input: expression = "1/3-1/2"
    Output: "-1/6"
*/
class Solution {
public:
    std::string fractionAddition(std::string expression) {
        int numerator = 0, denominator = 1, i = 0;
        while (i < expression.length()) {
            int sign = 1;
            if (expression[i] == '-') {
                sign = -1;
                ++i;
            }
            if (expression[i] == '+') {
                ++i;
            }

            int num = 0;
            while (i < expression.length() && isdigit(expression[i])) {
                num = (num * 10) + (expression[i] - '0');
                ++i;
            }
            num *= sign;

            ++i;

            int denom = 0;
            while (i < expression.length() && isdigit(expression[i])) {
                denom = (denom * 10) + (expression[i] - '0');
                ++i;
            }

            numerator = numerator * denom + num * denominator;
            denominator *= denom;
        }

        int gcd = GCD(abs(numerator), abs(denominator));
        numerator /= gcd;
        denominator /= gcd;

        return std::to_string(numerator) + "/" + std::to_string(denominator);
    }

private:
    int GCD(int a, int b) {
        while (b != 0) {
            int t = b;
            b = a % b;
            a = t;
        }
        return a;
    }
};
