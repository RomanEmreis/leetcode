/*
  773. Sliding Puzzle
  On an 2 x 3 board, there are five tiles labeled from 1 to 5, and an empty square represented by 0. A move consists of choosing 0 and a 4-directionally adjacent number and swapping it.
  The state of the board is solved if and only if the board is [[1,2,3],[4,5,0]].
  
  Given the puzzle board board, return the least number of moves required so that the state of the board is solved. If it is impossible for the state of the board to be solved, return -1.

  Example 1:
  Input: board = [[1,2,3],[4,0,5]]
  Output: 1
  Explanation: Swap the 0 and the 5 in one move.
*/
public class Solution {
    public int SlidingPuzzle(int[][] board) {
        string end = "123450";
        string start = Get(board);

        if (start == end) return 0;

        HashSet<string> visited = [start];
        Queue<string> q = [];
        q.Enqueue(start);

        int result = 0;

        while (q.Count > 0) {
            ++result;
            for (int n = q.Count; n > 0; --n) {
                string x = q.Dequeue();
                Set(x, board);
                foreach (string y in Next(board)) {
                    if (y == end) return result;
                    if (visited.Add(y)) q.Enqueue(y);
                }
            }
        }

        return -1;
    }

    private static string Get(int[][] board) {
        StringBuilder sb = new();
        for (int i = 0; i < 2; ++i) {
            for (int j = 0; j < 3; ++j) {
                sb.Append(board[i][j]);
            }
        }
        return sb.ToString();
    }

    private static void Set(string s, int[][] board) {
        for (int i = 0; i < 2; ++i) {
            for (int j = 0; j < 3; ++j) {
                board[i][j] = s[i * 3 + j] - '0';
            }
        }
    }

    private static IEnumerable<string> Next(int[][] board) {
        var (i, j) = Find0(board);

        int[] directions = [-1, 0, 1, 0, -1];
        for (int k = 0; k < 4; ++k) {
            int x = i + directions[k];
            int y = j + directions[k + 1];

            if (x >= 0 && x < 2 && y >= 0 && y < 3) {
                (board[i][j], board[x][y]) = (board[x][y], board[i][j]);
                yield return Get(board);
                (board[i][j], board[x][y]) = (board[x][y], board[i][j]);
            }
        }
    }

    private static (int, int) Find0(int[][] board) {
        for (int i = 0; i < 2; ++i) {
            for (int j = 0; j < 3; ++j) {
                if (board[i][j] == 0) return (i, j);
            }
        }
        return (0, 0);
    }
}
