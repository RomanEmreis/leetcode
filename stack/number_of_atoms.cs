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
public class Solution {
    public string CountOfAtoms(string formula) {
        Stack<Dictionary<string, int>> st = [];
        st.Push([]);

        int i = 0, n = formula.Length;
        while (i < n) {
            if (formula[i] == '(') {
                st.Push([]);
                ++i;
            } else if (formula[i] == ')') {
                Dictionary<string, int> top = st.Pop();
                ++i;

                int j = i;
                while (i < n && char.IsNumber(formula[i])) ++i;

                int multiplier = j < i ? int.Parse(formula[j..i]) : 1;
                Dictionary<string, int> dict = st.Peek();
                foreach (var (el, num) in top) {
                    if (dict.ContainsKey(el)) dict[el] += num * multiplier;
                    else dict[el] = num * multiplier;
                }
            } else {
                int j = i++;
                while (i < n && char.IsLower(formula[i])) ++i;

                string el = formula[j..i];
                j = i;
                while (i < n && char.IsNumber(formula[i])) ++i;

                int num = j < i ? int.Parse(formula[j..i]) : 1;
                
                Dictionary<string, int> dict = st.Peek();
                if (dict.ContainsKey(el)) dict[el] += num;
                else dict[el] = num;
            }
        }

        Dictionary<string, int> result = st.Pop();
        List<string> elements = [];
        foreach (var kv in result) {
            elements.Add(kv.Key);
        }
        elements.Sort();

        StringBuilder sb = new();
        foreach (string el in elements) {
            sb.Append(el);
            if (result[el] > 1) sb.Append(result[el]);
        }

        return sb.ToString();
    }
}
