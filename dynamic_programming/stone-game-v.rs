/*
  1563. Stone Game V
  
  There are several stones arranged in a row, and each stone has an associated value which is an integer given in the array stoneValue.
  
  In each round of the game, Alice divides the row into two non-empty rows (i.e. left row and right row), 
  then Bob calculates the value of each row which is the sum of the values of all the stones in this row. 
  Bob throws away the row which has the maximum value, and Alice's score increases by the value of the remaining row. If the value of the two rows are equal, 
  Bob lets Alice decide which row will be thrown away. The next round starts with the remaining row.
  
  The game ends when there is only one stone remaining. Alice's score is initially zero.
  
  Return the maximum score that Alice can obtain.
  
  Example 1:
  Input: stoneValue = [6,2,3,4,5,5]
  Output: 18
  Explanation: In the first round, Alice divides the row to [6,2,3], [4,5,5]. The left row has the value 11 and the right row has value 14. Bob throws away the right row and Alice's score is now 11.
  In the second round Alice divides the row to [6], [2,3]. This time Bob throws away the left row and Alice's score becomes 16 (11 + 5).
  The last round Alice has only one choice to divide the row which is [2], [3]. Bob throws away the right row and Alice's score is now 18 (16 + 2). The game ends because only one stone is remaining in the row.
  
  Example 2:
  Input: stoneValue = [7,7,7,7,7,7,7]
  Output: 28
  
  Example 3:
  Input: stoneValue = [4]
  Output: 0
*/
impl Solution {
    #[inline]
    fn pack(left: i32, right: i32) -> u64 {
        (left as u32 as u64)
            | ((right as u32 as u64) << 32)
    }

    #[inline]
    fn left(cell: u64) -> i32 {
        cell as u32 as i32
    }

    #[inline]
    fn right(cell: u64) -> i32 {
        (cell >> 32) as u32 as i32
    }

    pub fn stone_game_v(mut stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();

        for i in 1..n {
            stone_value[i] += stone_value[i - 1];
        }

        let mut offsets = [0usize; 501];

        for row in 0..n {
            offsets[row + 1] = offsets[row] + n - row;
        }

        let mut table = vec![0u64; offsets[n]];
        let mut res = 0;

        for i in (0..n).rev() {
            let before = if i == 0 {
                0
            } else {
                stone_value[i - 1]
            };

            let value = stone_value[i] - before;
            table[offsets[i]] = Self::pack(value, value);

            let mut boundary = i;
            let row_offset = offsets[i];

            for j in i + 1..n {
                let total = stone_value[j] - before;

                while boundary < j {
                    let left_sum = stone_value[boundary] - before;
                    if left_sum * 2 <= total {
                        boundary += 1;
                    } else {
                        break;
                    }
                }

                let mut best = 0;

                if boundary > i {
                    let cell = table[row_offset + boundary - 1 - i];

                    best = Self::left(cell);
                }

                let equality_at_last_left =
                    boundary > i
                    && (stone_value[boundary - 1] - before) * 2
                        == total;

                let right_start = if equality_at_last_left {
                    boundary
                } else {
                    boundary + 1
                };

                if right_start <= j {
                    let cell = table[
                        offsets[right_start] + j - right_start
                    ];

                    best = best.max(Self::right(cell));
                }

                let augmented = best + total;
                let current = row_offset + j - i;

                let left = Self::left(table[current - 1])
                    .max(augmented);

                let below = offsets[i + 1] + j - i - 1;
                let right = Self::right(table[below])
                    .max(augmented);

                table[current] = Self::pack(left, right);
                res = best;
            }
        }

        res
    }
}
