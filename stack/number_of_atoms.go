/*
  Given a string formula representing a chemical formula, return the count of each atom.
  The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.
  One or more digits representing that element's count may follow if the count is greater than 1. If the count is 1, no digits will follow.

  For example, "H2O" and "H2O2" are possible, but "H1O2" is impossible.
  Two formulas are concatenated together to produce another formula.

  For example, "H2O2He3Mg4" is also a formula.
  A formula placed in parentheses, and a count (optionally added) is also a formula.

  For example, "(H2O2)" and "(H2O2)3" are formulas.
  Return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than 1), 
  followed by the second name (in sorted order), followed by its count (if that count is more than 1), and so on.

  The test cases are generated so that all the values in the output fit in a 32-bit integer.

  Example 1:
    Input: formula = "H2O"
    Output: "H2O"
    Explanation: The count of elements are {'H': 2, 'O': 1}.
  
  Example 2:
    Input: formula = "Mg(OH)2"
    Output: "H2MgO2"
    Explanation: The count of elements are {'H': 2, 'Mg': 1, 'O': 2}.

  Example 3:
    Input: formula = "K4(ON(SO3)2)2"
    Output: "K4N2O14S4"
    Explanation: The count of elements are {'K': 4, 'N': 2, 'O': 14, 'S': 4}.
*/
func countOfAtoms(formula string) string {
    stack := []map[string]int{{}}
	i := 0
	n := len(formula)

	for i < n {
		if formula[i] == '(' {
			stack = append(stack, map[string]int{})
			i++
		} else if formula[i] == ')' {
			top := stack[len(stack) - 1]
			stack = stack[:len(stack) - 1]
			i++

			j := i

			for i < n && unicode.IsNumber(rune(formula[i])) {
				i++
			}

			multiplier, _ := strconv.Atoi(formula[j:i])

			if multiplier == 0 {
				multiplier = 1
			}

			dict := stack[len(stack) - 1]
			for el, num := range top {
				dict[el] += num * multiplier
			}
			stack[len(stack) - 1] = dict
		} else {
			j := i
			i++
			for i < n && unicode.IsLower(rune(formula[i])) {
				i++
			}

			el := formula[j:i]

			j = i
			for i < n && unicode.IsNumber(rune(formula[i])) {
				i++
			}

			num, _ := strconv.Atoi(formula[j:i])

			if num == 0 {
				num = 1
			}

			dict := stack[len(stack) - 1]

			dict[el] += num
			stack[len(stack) - 1] = dict
		}
	}

	result := stack[len(stack)-1]

	elements := []string{}

	for k, _ := range result {
		elements = append(elements, k)
	}

	sort.Strings(elements)

	builder := strings.Builder{}

	for _, el := range elements {
		builder.WriteString(el)
		if result[el] > 1 {
			builder.WriteString(strconv.Itoa(result[el]))
		}
	}

	return builder.String()
}
