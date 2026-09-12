/*
  3414. Maximum Score of Non-overlapping Intervals
  
  You are given a 2D integer array intervals, where intervals[i] = [li, ri, weighti]. Interval i starts at position li and ends at ri, and has a weight of weighti.
  You can choose up to 4 non-overlapping intervals. The score of the chosen intervals is defined as the total sum of their weights.
  
  Return the lexicographically smallest array of at most 4 indices from intervals with maximum score, representing your choice of non-overlapping intervals.
  
  Two intervals are said to be non-overlapping if they do not share any points. In particular, intervals sharing a left or right boundary are considered overlapping.
  
  Example 1:
  Input: intervals = [[1,3,2],[4,5,2],[1,5,5],[6,9,3],[6,7,1],[8,9,1]]
  Output: [2,3]
  Explanation:
  You can choose the intervals with indices 2, and 3 with respective weights of 5, and 3.
  
  Example 2:
  Input: intervals = [[5,8,1],[6,7,7],[4,7,3],[9,10,6],[7,8,2],[11,14,3],[3,5,5]]
  Output: [1,3,5,6]
  Explanation:
  You can choose the intervals with indices 1, 3, 5, and 6 with respective weights of 7, 6, 3, and 5.
*/
#[derive(Clone, Copy, Default)]
struct State {
    score: i64,
    indices: [u16; 4],
    len: u8,
}

impl State {
    #[inline]
    fn add(mut self, index: u16, weight: i32) -> Self {
        self.score += weight as i64;

        let mut position = self.len as usize;

        while position > 0 && self.indices[position - 1] > index {
            self.indices[position] = self.indices[position - 1];
            position -= 1;
        }

        self.indices[position] = index;
        self.len += 1;
        self
    }

    #[inline]
    fn better(self, other: Self) -> Self {
        if self.score != other.score {
            return if self.score > other.score {
                self
            } else {
                other
            };
        }

        let common = (self.len.min(other.len)) as usize;

        for i in 0..common {
            if self.indices[i] != other.indices[i] {
                return if self.indices[i] < other.indices[i] {
                    self
                } else {
                    other
                };
            }
        }

        if self.len < other.len {
            self
        } else {
            other
        }
    }
}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut items: Vec<(i32, i32, i32, u16)> = intervals
            .iter()
            .enumerate()
            .map(|(index, value)| {
                (
                    value[0],
                    value[1],
                    value[2],
                    index as u16,
                )
            })
            .collect();

        items.sort_unstable_by_key(|item| item.1);

        let n = items.len();
        let ends: Vec<i32> = items.iter().map(|item| item.1).collect();
        let mut predecessor = Vec::with_capacity(n);

        for i in 0..n {
            predecessor.push(
                ends[..i].partition_point(|&end| end < items[i].0)
            );
        }

        let mut storage = vec![State::default(); 2 * (n + 1)];
        let (mut previous, mut current) =
            storage.split_at_mut(n + 1);

        for _ in 0..4 {
            current[0] = State::default();

            for i in 1..=n {
                let item = items[i - 1];

                let take = previous[predecessor[i - 1]]
                    .add(item.3, item.2);

                current[i] = take.better(current[i - 1]);
            }

            std::mem::swap(&mut previous, &mut current);
        }

        previous[n].indices[..previous[n].len as usize]
            .iter()
            .map(|&index| index as i32)
            .collect()
    }
}
