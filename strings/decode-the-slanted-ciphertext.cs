/*
  2075. Decode the Slanted Ciphertext
  
  A string originalText is encoded using a slanted transposition cipher to a string encodedText with the help of a matrix having a fixed number of rows rows.
  originalText is placed first in a top-left to bottom-right manner.
  
  The blue cells are filled first, followed by the red cells, then the yellow cells, and so on, until we reach the end of originalText. The arrow indicates the order in which the cells are filled. 
  All empty cells are filled with ' '. The number of columns is chosen such that the rightmost column will not be empty after filling in originalText.
  encodedText is then formed by appending all characters of the matrix in a row-wise fashion.
  
  The characters in the blue cells are appended first to encodedText, then the red cells, and so on, and finally the yellow cells. The arrow indicates the order in which the cells are accessed.
  For example, if originalText = "cipher" and rows = 3, then we encode it in the following manner:
  The blue arrows depict how originalText is placed in the matrix, and the red arrows denote the order in which encodedText is formed. In the above example, encodedText = "ch ie pr".
  Given the encoded string encodedText and number of rows rows, return the original string originalText.
  
  Note: originalText does not have any trailing spaces ' '. The test cases are generated such that there is only one possible originalText.
  
  Example 1:
  Input: encodedText = "ch   ie   pr", rows = 3
  Output: "cipher"
  Explanation: This is the same example described in the problem description.
  
  Example 2:
  Input: encodedText = "iveo    eed   l te   olc", rows = 4
  Output: "i love leetcode"
  Explanation: The figure above denotes the matrix that was used to encode originalText. 
  The blue arrows show how we can find originalText from encodedText.
  
  Example 3:
  Input: encodedText = "coding", rows = 1
  Output: "coding"
  Explanation: Since there is only 1 row, both originalText and encodedText are the same.
*/
public class Solution {
    public string DecodeCiphertext(string encodedText, int rows) {
        int n = encodedText.Length;
        if (rows == 0 || n == 0)
            return "";

        int cols = n / rows;
        Span<char> res = stackalloc char[n];

        for (int j = 0, k = 0; j < cols; ++j) {
            for (int i = 0; i < rows; ++i) {
                int col = i + j;
                if (col >= cols)
                    break;

                res[k++] = encodedText[i * cols + col];
            }
        }

        int t = 0;
        for (int i = n - 1; i >= 0; --i) {
            if (res[i] == ' ' || res[i] == '\0')
                ++t;
            else
                break;
        }

        return new string(res[..(n - t)]);
    }
}
