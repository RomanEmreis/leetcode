/*
  835. Image Overlap
  
  You are given two images, img1 and img2, represented as binary, square matrices of size n x n. A binary matrix has only 0s and 1s as values.
  We translate one image however we choose by sliding all the 1 bits left, right, up, and/or down any number of units. We then place it on top of the other image. 
  We can then calculate the overlap by counting the number of positions that have a 1 in both images.
  
  Note also that a translation does not include any kind of rotation. Any 1 bits that are translated outside of the matrix borders are erased.
  
  Return the largest possible overlap.
  
  Example 1:
  Input: img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
  Output: 3
  Explanation: We translate img1 to right by 1 unit and down by 1 unit.
  The number of positions that have a 1 in both images is 3 (shown in red).
  
  Example 2:
  Input: img1 = [[1]], img2 = [[1]]
  Output: 1
  
  Example 3:
  Input: img1 = [[0]], img2 = [[0]]
  Output: 0
*/
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len() as i32;
        let mut rows1 = [0u32; 30];
        let mut rows2 = [0u32; 30];

        for row in 0..n as usize {
            let mut bits1 = 0u32;
            let mut bits2 = 0u32;

            for column in 0..n as usize {
                bits1 |= (img1[row][column] as u32) << column;
                bits2 |= (img2[row][column] as u32) << column;
            }

            rows1[row] = bits1;
            rows2[row] = bits2;
        }

        let mut res = 0u32;

        for row_shift in -(n - 1)..=(n - 1) {
            let first_row = (-row_shift).max(0);
            let last_row = n.min(n - row_shift);

            for column_shift in -(n - 1)..=(n - 1) {
                let mut overlap = 0u32;

                for row in first_row..last_row {
                    let source = rows1[row as usize];

                    let shifted = if column_shift >= 0 {
                        source << column_shift
                    } else {
                        source >> -column_shift
                    };

                    overlap += (shifted & rows2[(row + row_shift) as usize]).count_ones();
                }

                res = res.max(overlap);
            }
        }

        res as i32
    }
}
