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
func fractionAddition(expression string) string {
    i, n, numerator, denominator := 0, len(expression), 0, 1;
    for i < n {
        sign := 1;
        if expression[i] == '-' {
            sign = -1;
            i++;
        }
        if expression[i] == '+' {
            i++;
        }

        num := 0;
        for i < n && unicode.IsDigit(rune(expression[i])) {
            num = (num * 10) + int(expression[i] - '0');
            i++;
        }
        num *= sign;

        i++;

        denom := 0;
        for i < n && unicode.IsDigit(rune(expression[i])) {
            denom = (denom * 10) + int(expression[i] - '0');
            i++;
        }

        numerator = numerator * denom + num * denominator;
        denominator *= denom;
    }
    gcd := gcd(int(math.Abs(float64(numerator))), int(math.Abs(float64(denominator))));
    numerator /= gcd;
    denominator /= gcd;
    return strconv.Itoa(numerator) + "/" + strconv.Itoa(denominator);
}

func gcd(a,b int) int {
    for b != 0 {
        b, a = a % b, b;
    }
    return a;
}
