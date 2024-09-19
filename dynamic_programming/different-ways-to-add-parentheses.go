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
func diffWaysToCompute(expression string) []int {
    if isOnlyDigits(expression) {
		num, _ := strconv.Atoi(expression);
		return []int{num};
	}

	var results []int
	for i := 0; i < len(expression); i++ {
		if expression[i] == '+' || expression[i] == '-' || expression[i] == '*' {
			leftResults := diffWaysToCompute(expression[:i]);
			rightResults := diffWaysToCompute(expression[i+1:]);

			for _, left := range leftResults {
				for _, right := range rightResults {
					if expression[i] == '+' {
						results = append(results, left+right);
					} else if expression[i] == '-' {
						results = append(results, left-right);
					} else if expression[i] == '*' {
						results = append(results, left*right);
					}
				}
			}
		}
	}
	return results;
}

func isOnlyDigits(s string) bool {
	for _, r := range s {
		if !unicode.IsDigit(r) {
			return false;
		}
	}
	return true;
}
