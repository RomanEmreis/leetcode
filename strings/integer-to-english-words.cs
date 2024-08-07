/*
  Convert a non-negative integer num to its English words representation.
  
  Example:
    Input: num = 123
    Output: "One Hundred Twenty Three"
*/
public class Solution {
    private static readonly string[] singles = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
    private static readonly string[] teens = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
    private static readonly string[] tens = ["", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
    private static readonly string[] thousands = ["", "Thousand", "Million", "Billion"];

    private const string space = " ", hundred = " Hundred ", zero = "Zero";

    public string NumberToWords(int num) {
        if (num == 0) return zero;

        const int t = 1000;

        StringBuilder result = new();
        for (int i = 0; num > 0; ++i) {
            if (num % t != 0) {
                result.Insert(0, Convert(num % t) + thousands[i] + space);
            }
            num /= t;
        }

        return result.ToString().Trim();

        static string Convert(int n) => n switch {
            0 => string.Empty,
            < 10 => singles[n] + space,
            < 20 => teens[n - 10] + space,
            < 100 => tens[n / 10] + space + Convert(n % 10),
            _ => singles[n / 100] + hundred + Convert(n % 100)
        };
    }
}
