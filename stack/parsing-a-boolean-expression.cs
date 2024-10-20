/*
  A boolean expression is an expression that evaluates to either true or false. It can be in one of the following shapes:
  
  't' that evaluates to true.
  'f' that evaluates to false.
  '!(subExpr)' that evaluates to the logical NOT of the inner expression subExpr.
  '&(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical AND of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
  '|(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical OR of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
  Given a string expression that represents a boolean expression, return the evaluation of that expression.
  
  It is guaranteed that the given expression is valid and follows the given rules.
  
  Example 1:
    Input: expression = "&(|(f))"
    Output: false
    Explanation: 
    First, evaluate |(f) --> f. The expression is now "&(f)".
    Then, evaluate &(f) --> f. The expression is now "f".
    Finally, return false.
  
  Example 2:
    Input: expression = "|(f,f,f,t)"
    Output: true
    Explanation: The evaluation of (false OR false OR false OR true) is true.
  
  Example 3:
    Input: expression = "!(&(f,t))"
    Output: true
    Explanation: 
    First, evaluate &(f,t) --> (false AND true) --> false --> f. The expression is now "!(f)".
    Then, evaluate !(f) --> NOT false --> true. We return true.
*/
public class Solution {
    public bool ParseBoolExpr(string expression) {
        Stack<char> operators = new();
        Stack<List<bool>> operands = new();

        int i = 0;
        while (i < expression.Length) {
            if (expression[i] == 't' || expression[i] == 'f') {
                bool value = expression[i] == 't';
                if (operands.Count > 0) operands.Peek().Add(value);
                else operands.Push([value]);
                ++i;
            } else if (expression[i] == '&' || expression[i] == '|' || expression[i] == '!') {
                operators.Push(expression[i]);
                operands.Push([]);
                i += 2;
            } else if (expression[i] == ')') {
                char op = operators.Pop();
                List<bool> currentOperands = operands.Pop();
                
                bool result = false;
                if (op == '&') result = currentOperands.All(x => x);
                else if (op == '|') result = currentOperands.Any(x => x);
                else result = !currentOperands[0];

                if (operands.Count > 0) operands.Peek().Add(result);
                else operands.Push([result]);
                ++i;
            } else {
                ++i;
            }
        }

        return operands.Pop().First();
    }
}
