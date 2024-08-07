/*
  Convert a non-negative integer num to its English words representation.
  
  Example:
    Input: num = 123
    Output: "One Hundred Twenty Three"
*/
class Solution {
private:
    vector<string> singles = {"", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"};
    vector<string> teens = {"Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"};
    vector<string> tens = {"", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"};
    vector<string> thousands = {"", "Thousand", "Million", "Billion"};

    string convert(int n) {
        string result;
        if (n >= 100) {
            result += singles[n / 100] + " Hundred ";
            n %= 100;
        }
        if (n >= 20) {
            result += tens[n / 10] + " ";
            n %= 10;
        }
        if (n >= 10) {
            result += teens[n - 10] + " ";
        } else if (n > 0) {
            result += singles[n] + " ";
        }
        return result;
    }
public:
    string numberToWords(int num) {
        if (num == 0) return "Zero";

        string result;
        for (int i = 0; num > 0; ++i) {
            if (num % 1000 != 0) {
                result = convert(num % 1000) + thousands[i] + " " + result;;
            }
            num /= 1000;
        }

        while (!result.empty() && result[result.size() - 1] == ' ') {
            result.pop_back();
        }

        return result;
    }
};
