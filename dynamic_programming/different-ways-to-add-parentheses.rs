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
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        if expression.chars().all(char::is_numeric) {
            return vec![expression.parse::<i32>().unwrap()];
        }

        let mut results = Vec::new();

        for (i, c) in expression.chars().enumerate() {
            match c {
                '+' | '-' | '*' => {
                    let left_results = Self::diff_ways_to_compute(expression[..i].to_string());
                    let right_results = Self::diff_ways_to_compute(expression[i+1..].to_string());

                    for &left in &left_results {
                        for &right in &right_results {
                            let result = match c {
                                '+' => left + right,
                                '-' => left - right,
                                '*' => left * right,
                                _ => unreachable!(),
                            };
                            results.push(result);
                        }
                    }
                },
                _ => {}
            }
        }

        results
    }
}
