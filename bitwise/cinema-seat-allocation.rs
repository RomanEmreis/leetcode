/*
  1386. Cinema Seat Allocation
  
  A cinema has n rows of seats, numbered from 1 to n. Each row has 10 seats, numbered from 1 to 10.
  You are given a 2D integer array reservedSeats, where reservedSeats[i] = [rowi, seati] means that seat seati in row rowi is already reserved.
  
  A four-person group must be assigned to four seats in the same row. The group can be seated in one of the following seat blocks:
      seats 2, 3, 4, 5
      seats 4, 5, 6, 7
      seats 6, 7, 8, 9
  
  A block can be used only if none of its seats are reserved. Each seat can be assigned to at most one group.
  
  Return an integer denoting the maximum number of four-person groups that can be assigned.
  
  Example 1:
  Input: n = 3, reservedSeats = [[1,2],[1,3],[1,8],[2,6],[3,1],[3,10]]
  Output: 4
  Explanation: The figure above shows an optimal allocation of four groups. Seats marked in blue are already reserved, 
  and each set of four contiguous seats marked in orange is assigned to one group.
  
  Example 2:
  Input: n = 2, reservedSeats = [[2,1],[1,8],[2,6]]
  Output: 2
  
  Example 3:
  Input: n = 4, reservedSeats = [[4,3],[1,4],[4,6],[1,7]]
  Output: 4
*/
use std::collections::HashMap;

const LEFT: u8 = 0b0000_1111;
const MIDDLE: u8 = 0b0011_1100;
const RIGHT: u8 = 0b1111_0000;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut rows = HashMap::<i32, u8>::with_capacity(reserved_seats.len());
        for reservation in reserved_seats {
            let row = reservation[0];
            let seat = reservation[1];

            if seat >= 2 && seat <= 9 {
                let bit = 1u8 << (seat - 2) as u32;
                *rows.entry(row).or_insert(0) |= bit;
            }
        }

        let mut res = 2 * (n - rows.len() as i32);

        for &mask in rows.values() {
            let left_free = mask & LEFT == 0;
            let middle_free = mask & MIDDLE == 0;
            let right_free = mask & RIGHT == 0;

            if left_free && right_free {
                res += 2;
            } else if left_free || middle_free || right_free {
                res += 1;
            }
        }

        res
    }
}
