/*
  1432. Max Difference You Can Get From Changing an Integer
  
  You are given an integer num. You will apply the following steps to num two separate times:
  Pick a digit x (0 <= x <= 9).
  Pick another digit y (0 <= y <= 9). Note y can be equal to x.
  Replace all the occurrences of x in the decimal representation of num by y.
  Let a and b be the two results from applying the operation to num independently.
  
  Return the max difference between a and b.
  
  Note that neither a nor b may have any leading zeros, and must not be 0.
  
  Example 1:
  Input: num = 555
  Output: 888
  Explanation: The first time pick x = 5 and y = 9 and store the new integer in a.
  The second time pick x = 5 and y = 1 and store the new integer in b.
  We have now a = 999 and b = 111 and max difference = 888
  
  Example 2:
  Input: num = 9
  Output: 8
  Explanation: The first time pick x = 9 and y = 9 and store the new integer in a.
  The second time pick x = 9 and y = 1 and store the new integer in b.
  We have now a = 9 and b = 1 and max difference = 8
*/
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut a = to_digits(num);
        let mut b = a.clone();

        for i in 0..a.len() {
            if a[i] != 9 {
                let d = a[i];
                replace(&mut a, d, 9);
                break;
            }
        }

        if b[0] != 1 {
            let d = b[0];
            replace(&mut b, d, 1);
        } else {
            for i in 1..b.len() {
                let d = b[i];
                if d != 0 && d != 1 {
                    replace(&mut b, d, 0);
                    break;
                }
            }
        }

        to_int(&a) - to_int(&b)
    }
}

fn replace(digits: &mut Vec<i32>, from: i32, to: i32) {
    for d in digits.iter_mut() {
        if *d == from {
            *d = to;
        }
    }
}

fn to_int(digits: &[i32]) -> i32 {
    digits
        .iter()
        .fold(0, |acc, &d| acc * 10 + d)
}

fn to_digits(mut num: i32) -> Vec<i32> {
    let mut digits = vec![];
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
    digits
}
