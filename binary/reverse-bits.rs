/*
  190. Reverse Bits
  
  Reverse bits of a given 32 bits signed integer.
  
  Example 1:
  Input: n = 43261596
  Output: 964176192
  Explanation:
  Integer	  Binary
  43261596	00000010100101000001111010011100
  964176192	00111001011110000010100101000000
  
  Example 2:
  Input: n = 2147483644
  Output: 1073741822
  Explanation:
  Integer	    Binary
  2147483644	01111111111111111111111111111100
  1073741822	00111111111111111111111111111110
*/
impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut res: u32 = 0;
        let mut n = n as u32;

        for _ in 0..32 {
            res = (res << 1) | (n & 1);
            n >>= 1;
        }

        res as i32
    }
}
