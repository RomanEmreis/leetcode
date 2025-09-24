/*
  166. Fraction to Recurring Decimal
  
  Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.
  If the fractional part is repeating, enclose the repeating part in parentheses.
  If multiple answers are possible, return any of them.
  It is guaranteed that the length of the answer string is less than 104 for all the given inputs.
  
  Example 1:
  Input: numerator = 1, denominator = 2
  Output: "0.5"
  
  Example 2:
  Input: numerator = 2, denominator = 1
  Output: "2"
  
  Example 3:
  Input: numerator = 4, denominator = 333
  Output: "0.(012)"
*/
use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }

        let mut res = String::new();
        if (numerator < 0) ^ (denominator < 0) {
            res.push('-');
        }

        let n = (numerator as i64).abs();
        let d = (denominator as i64).abs();

        res.push_str(&(n / d).to_string());

        if n % d == 0 {
            return res;
        }

        res.push('.');
        
        let mut seen = HashMap::new();
        let mut r = n % d;
        while r > 0 {
            if let Some(&rem) = seen.get(&r) {
                res.insert(rem, '(');
                res.push(')');
                break;
            }
            seen.insert(r, res.len());
            r *= 10;
            res.push_str(&(r / d).to_string());
            r %= d;
        }

        res
    }
}
