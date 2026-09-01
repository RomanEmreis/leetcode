/*
  3568. Minimum Moves to Clean the Classroom
  
  You are given an m x n grid classroom where a student volunteer is tasked with cleaning up litter scattered around the room. Each cell in the grid is one of the following:
      'S': Starting position of the student
      'L': Litter that must be collected (once collected, the cell becomes empty)
      'R': Reset area that restores the student's energy to full capacity, regardless of their current energy level (can be used multiple times)
      'X': Obstacle the student cannot pass through
      '.': Empty space
  
  You are also given an integer energy, representing the student's maximum energy capacity. The student starts with this energy from the starting position 'S'.
  
  Each move to an adjacent cell (up, down, left, or right) costs 1 unit of energy. If the energy reaches 0, the student can only continue if they are on a reset area 'R', which resets the energy to its maximum capacity energy.
  
  Return the minimum number of moves required to collect all litter items, or -1 if it's impossible.
   
  Example 1:
  Input: classroom = ["S.", "XL"], energy = 2
  Output: 2
  Explanation:
      The student starts at cell (0, 0) with 2 units of energy.
      Since cell (1, 0) contains an obstacle 'X', the student cannot move directly downward.
      A valid sequence of moves to collect all litter is as follows:
          Move 1: From (0, 0) → (0, 1) with 1 unit of energy and 1 unit remaining.
          Move 2: From (0, 1) → (1, 1) to collect the litter 'L'.
      The student collects all the litter using 2 moves. Thus, the output is 2.
  
  Example 2:
  Input: classroom = ["LS", "RL"], energy = 4
  Output: 3
  Explanation:
      The student starts at cell (0, 1) with 4 units of energy.
      A valid sequence of moves to collect all litter is as follows:
          Move 1: From (0, 1) → (0, 0) to collect the first litter 'L' with 1 unit of energy used and 3 units remaining.
          Move 2: From (0, 0) → (1, 0) to 'R' to reset and restore energy back to 4.
          Move 3: From (1, 0) → (1, 1) to collect the second litter 'L'.
      The student collects all the litter using 3 moves. Thus, the output is 3.
  
  Example 3:
  Input: classroom = ["L.S", "RXL"], energy = 3
  Output: -1
  Explanation:
  No valid path collects all 'L'.
*/
use std::collections::VecDeque;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let rows = classroom.len();
        let cols = classroom[0].len();
        let cells = rows * cols;

        let mut grid = Vec::with_capacity(cells);
        for row in &classroom {
            grid.extend_from_slice(row.as_bytes());
        }

        let mut start_row = 0usize;
        let mut start_col = 0usize;
        let mut litter_bits = vec![0u16; cells];
        let mut litter_count = 0u32;

        for cell in 0..cells {
            match grid[cell] {
                b'S' => {
                    start_row = cell / cols;
                    start_col = cell % cols;
                }
                b'L' => {
                    litter_bits[cell] =
                        1u16 << litter_count;
                    litter_count += 1;
                }
                _ => {}
            }
        }

        let full_mask = (1u32 << litter_count) - 1;
        let mask_count = 1usize << litter_count;

        let mut best_energy = vec![-1i8; mask_count * cells];

        let start_cell = start_row * cols + start_col;
        best_energy[start_cell] = energy as i8;

        #[inline]
        fn pack(
            row: u32,
            col: u32,
            mask: u32,
            remaining: u32,
        ) -> u32 {
            row | (col << 5) | (mask << 10) | (remaining << 20)
        }

        let mut queue = VecDeque::<u32>::new();
        queue.push_back(pack(
            start_row as u32,
            start_col as u32,
            0,
            energy as u32,
        ));

        let mut moves = 0;

        while !queue.is_empty() {
            let level_size = queue.len();

            for _ in 0..level_size {
                let state = queue.pop_front().unwrap();

                let row = (state & 31) as usize;
                let col = ((state >> 5) & 31) as usize;
                let mask = (state >> 10) & 1023;
                let remaining = state >> 20;

                if mask == full_mask {
                    return moves;
                }

                if remaining == 0 {
                    continue;
                }

                let mut neighbors = [(0usize, 0usize); 4];
                let mut count = 0usize;

                if row > 0 {
                    neighbors[count] = (row - 1, col);
                    count += 1;
                }
                if row + 1 < rows {
                    neighbors[count] = (row + 1, col);
                    count += 1;
                }
                if col > 0 {
                    neighbors[count] = (row, col - 1);
                    count += 1;
                }
                if col + 1 < cols {
                    neighbors[count] = (row, col + 1);
                    count += 1;
                }

                for &(next_row, next_col) in &neighbors[..count] {
                    let next_cell = next_row * cols + next_col;
                    let tile = grid[next_cell];

                    if tile == b'X' {
                        continue;
                    }

                    let next_mask =
                        mask | litter_bits[next_cell] as u32;

                    let next_energy = if tile == b'R' {
                        energy as u32
                    } else {
                        remaining - 1
                    };

                    let key =
                        next_mask as usize * cells + next_cell;

                    if next_energy as i8 <= best_energy[key] {
                        continue;
                    }

                    best_energy[key] = next_energy as i8;
                    queue.push_back(pack(
                        next_row as u32,
                        next_col as u32,
                        next_mask,
                        next_energy,
                    ));
                }
            }

            moves += 1;
        }

        -1
    }
}
