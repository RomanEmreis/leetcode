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
class Solution {
public:
    string countOfAtoms(string formula) {
        stack<map<string, int>> st;
        st.push({});

        int i = 0, n = formula.length();

        while(i < n) {
            if(formula[i] == '(') {
                st.push({});
                i++;
            } else if(formula[i] == ')') {
                map<string, int> top = st.top();
                st.pop();
                i++;

                int j = i;
                while(i < n && isdigit(formula[i])) i++;

                int multiplier = j < i ? stoi(formula.substr(j, i-j)) : 1;

                map<string, int>& dict = st.top();
                for(auto const& pair: top) {
                    dict[pair.first] += pair.second * multiplier;
                }
            } else {
                int j = i++;
                while(i < n && islower(formula[i])) i++;

                string el = formula.substr(j, i-j);
                j = i;
                while(i < n && isdigit(formula[i])) i++;

                int num = j < i ? std::stoi(formula.substr(j, i-j)) : 1;

                st.top()[el] += num;
            }
        }

        map<string, int> result = st.top();
        st.pop();
        vector<string> elements;
        for(auto const& pair: result){
            elements.push_back(pair.first);
        }
        sort(elements.begin(), elements.end());

        stringstream sb;
        for(string const& el: elements){
            sb << el;
            if(result[el] > 1) sb << result[el];
        }

        return sb.str();
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
