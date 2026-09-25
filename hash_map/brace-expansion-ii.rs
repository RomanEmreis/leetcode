/*
  1096. Brace Expansion II
  
  Under the grammar given below, strings can represent a set of lowercase words. Let R(expr) denote the set of words the expression represents.
  
  The grammar can best be understood through simple examples:
      Single letters represent a singleton set containing that word.
          R("a") = {"a"}
          R("w") = {"w"}
      When we take a comma-delimited list of two or more expressions, we take the union of possibilities.
          R("{a,b,c}") = {"a","b","c"}
          R("{{a,b},{b,c}}") = {"a","b","c"} (notice the final set only contains each word at most once)
      When we concatenate two expressions, we take the set of possible concatenations between two words where the first word comes from the first expression and the second word comes from the second expression.
          R("{a,b}{c,d}") = {"ac","ad","bc","bd"}
          R("a{b,c}{d,e}f{g,h}") = {"abdfg", "abdfh", "abefg", "abefh", "acdfg", "acdfh", "acefg", "acefh"}
  
  Formally, the three rules for our grammar:
      For every lowercase letter x, we have R(x) = {x}.
      For expressions e1, e2, ... , ek with k >= 2, we have R({e1, e2, ...}) = R(e1) ∪ R(e2) ∪ ...
      For expressions e1 and e2, we have R(e1 + e2) = {a + b for (a, b) in R(e1) × R(e2)}, where + denotes concatenation, and × denotes the cartesian product.
  
  Given an expression representing a set of words under the given grammar, return the sorted list of words that the expression represents.
   
  Example 1:
  Input: expression = "{a,b}{c,{d,e}}"
  Output: ["ac","ad","ae","bc","bd","be"]
  
  Example 2:
  Input: expression = "{{a,z},a{b,c},{ab,z}}"
  Output: ["a","ab","ac","z"]
  Explanation: Each distinct word is written only once in the final answer.
*/
use std::collections::HashSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        struct Parser<'a> {
            bytes: &'a [u8],
            position: usize,
        }

        impl<'a> Parser<'a> {
            fn parse_expression(&mut self) -> HashSet<String> {
                let mut result = self.parse_term();

                while self.position < self.bytes.len()
                    && self.bytes[self.position] == b','
                {
                    self.position += 1;
                    let mut other = self.parse_term();

                    if result.len() < other.len() {
                        std::mem::swap(&mut result, &mut other);
                    }

                    result.extend(other);
                }

                result
            }

            fn parse_term(&mut self) -> HashSet<String> {
                let mut result = HashSet::new();
                result.insert(String::new());

                while self.position < self.bytes.len()
                    && self.bytes[self.position] != b','
                    && self.bytes[self.position] != b'}'
                {
                    let factor = if self.bytes[self.position] == b'{' {
                        self.position += 1;
                        let nested = self.parse_expression();
                        self.position += 1; // `}`
                        nested
                    } else {
                        let start = self.position;

                        while self.position < self.bytes.len()
                            && self.bytes[self.position].is_ascii_lowercase()
                        {
                            self.position += 1;
                        }

                        let literal =
                            String::from_utf8(
                                self.bytes[start..self.position].to_vec()
                            ).unwrap();

                        HashSet::from([literal])
                    };

                    result = multiply(result, factor);
                }

                result
            }
        }

        fn multiply(left: HashSet<String>, right: HashSet<String>) -> HashSet<String> {
            let capacity = left.len().saturating_mul(right.len());
            let mut result = HashSet::with_capacity(capacity);

            for prefix in &left {
                for suffix in &right {
                    let mut word = String::with_capacity(prefix.len() + suffix.len());

                    word.push_str(prefix);
                    word.push_str(suffix);

                    result.insert(word);
                }
            }

            result
        }

        let mut parser = Parser {
            bytes: expression.as_bytes(),
            position: 0,
        };

        let mut result: Vec<String> = parser
            .parse_expression()
            .into_iter()
            .collect();

        result.sort_unstable();
        result
    }
}
