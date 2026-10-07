/*
  301. Remove Invalid Parentheses
  
  Given a string s that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.
  
  Return a list of unique strings that are valid with the minimum number of removals. You may return the answer in any order.
  
  Example 1:
  Input: s = "()())()"
  Output: ["(())()","()()()"]
  
  Example 2:
  Input: s = "(a)())()"
  Output: ["(a())()","(a)()()"]
  
  Example 3:
  Input: s = ")("
  Output: [""]
*/
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let bytes = s.as_bytes();

        let mut remove_open = 0;
        let mut remove_close = 0;

        for &byte in bytes {
            match byte {
                b'(' => remove_open += 1,
                b')' if remove_open > 0 => remove_open -= 1,
                b')' => remove_close += 1,
                _ => {}
            }
        }

        let mut res = Vec::new();
        let mut path = Vec::with_capacity(bytes.len());

        fn search(
            bytes: &[u8],
            index: usize,
            balance: i32,
            remove_open: i32,
            remove_close: i32,
            previous_removed: bool,
            path: &mut Vec<u8>,
            res: &mut Vec<String>,
        ) {
            let remaining = bytes.len() - index;
            if remaining < (remove_open + remove_close) as usize {
                return;
            }

            if index == bytes.len() {
                if balance == 0 && remove_open == 0 && remove_close == 0
                {
                    res.push(String::from_utf8(path.clone()).unwrap());
                }

                return;
            }

            let byte = bytes[index];

            match byte {
                b'(' => {
                    let can_remove =
                        remove_open > 0
                        && (index == 0
                            || bytes[index - 1] != b'('
                            || previous_removed);

                    if can_remove {
                        search(
                            bytes,
                            index + 1,
                            balance,
                            remove_open - 1,
                            remove_close,
                            true,
                            path,
                            res,
                        );
                    }

                    path.push(byte);

                    search(
                        bytes,
                        index + 1,
                        balance + 1,
                        remove_open,
                        remove_close,
                        false,
                        path,
                        res,
                    );

                    path.pop();
                }

                b')' => {
                    let can_remove =
                        remove_close > 0
                        && (index == 0
                            || bytes[index - 1] != b')'
                            || previous_removed);

                    if can_remove {
                        search(
                            bytes,
                            index + 1,
                            balance,
                            remove_open,
                            remove_close - 1,
                            true,
                            path,
                            res,
                        );
                    }

                    if balance > 0 {
                        path.push(byte);

                        search(
                            bytes,
                            index + 1,
                            balance - 1,
                            remove_open,
                            remove_close,
                            false,
                            path,
                            res,
                        );

                        path.pop();
                    }
                }

                _ => {
                    path.push(byte);

                    search(
                        bytes,
                        index + 1,
                        balance,
                        remove_open,
                        remove_close,
                        false,
                        path,
                        res,
                    );

                    path.pop();
                }
            }
        }

        search(
            bytes,
            0,
            0,
            remove_open,
            remove_close,
            false,
            &mut path,
            &mut res,
        );

        res
    }
}
