/*
  3505. Minimum Operations to Make Elements Within K Subarrays Equal
  
  You are given an integer array nums and two integers, x and k. You can perform the following operation any number of times (including zero):
      Increase or decrease any element of nums by 1.
  
  Return the minimum number of operations needed to have at least k non-overlapping subarrays of size exactly x in nums, where all elements within each subarray are equal.
  
  Example 1:
  Input: nums = [5,-2,1,3,7,3,6,4,-1], x = 3, k = 2
  Output: 8
  Explanation:
      Use 3 operations to add 3 to nums[1] and use 2 operations to subtract 2 from nums[3]. The resulting array is [5, 1, 1, 1, 7, 3, 6, 4, -1].
      Use 1 operation to add 1 to nums[5] and use 2 operations to subtract 2 from nums[6]. The resulting array is [5, 1, 1, 1, 7, 4, 4, 4, -1].
      Now, all elements within each subarray [1, 1, 1] (from indices 1 to 3) and [4, 4, 4] (from indices 5 to 7) are equal. Since 8 total operations were used, 8 is the output.
  
  Example 2:
  Input: nums = [9,-2,-2,-2,1,5], x = 2, k = 2
  Output: 3
  Explanation:
      Use 3 operations to subtract 3 from nums[4]. The resulting array is [9, -2, -2, -2, -2, 5].
      Now, all elements within each subarray [-2, -2] (from indices 1 to 2) and [-2, -2] (from indices 3 to 4) are equal. Since 3 operations were used, 3 is the output.
*/
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let width = x as usize;
        let groups = k as usize;

        let mut sorted = nums.clone();
        sorted.sort_unstable();

        let ranks: Vec<usize> = nums
            .iter()
            .map(|value| sorted.partition_point(|item| item < value) + 1)
            .collect();

        let mut fenwick = Fenwick::new(n);
        let mut total_sum = 0i64;

        for index in 0..width {
            let value = nums[index] as i64;
            fenwick.add(ranks[index], 1, value);
            total_sum += value;
        }

        let window_count = n - width + 1;
        let mut costs = vec![0i64; window_count];
        let median_order = (width as i32 + 1) / 2;

        for start in 0..window_count {
            let median_rank = fenwick.kth(median_order);
            let median = sorted[median_rank - 1] as i64;

            let (left_count, left_sum) =
                fenwick.prefix(median_rank);

            let right_count = width as i64 - left_count as i64;
            let right_sum = total_sum - left_sum;

            costs[start] =
                median * left_count as i64 - left_sum
                + right_sum
                - median * right_count;

            if start + width < n {
                let outgoing = nums[start] as i64;
                let incoming = nums[start + width] as i64;

                fenwick.add(ranks[start], -1, -outgoing);
                fenwick.add(ranks[start + width], 1, incoming);

                total_sum += incoming - outgoing;
            }
        }

        const INF: i64 = i64::MAX / 4;

        let mut previous = vec![0i64; n + 1];
        let mut current = vec![INF; n + 1];

        for _ in 0..groups {
            current.fill(INF);

            for prefix in 1..=n {
                current[prefix] = current[prefix - 1];

                if prefix >= width && previous[prefix - width] < INF {
                    current[prefix] = current[prefix].min(
                        previous[prefix - width]
                            + costs[prefix - width],
                    );
                }
            }

            std::mem::swap(&mut previous, &mut current);
        }

        previous[n]
    }
}

struct Fenwick {
    counts: Vec<i32>,
    sums: Vec<i64>,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            counts: vec![0; size + 1],
            sums: vec![0; size + 1],
        }
    }

    fn add(
        &mut self,
        mut index: usize,
        count_delta: i32,
        sum_delta: i64,
    ) {
        while index < self.counts.len() {
            self.counts[index] += count_delta;
            self.sums[index] += sum_delta;
            index += index & index.wrapping_neg();
        }
    }

    fn prefix(&self, mut index: usize) -> (i32, i64) {
        let mut count = 0;
        let mut sum = 0i64;

        while index > 0 {
            count += self.counts[index];
            sum += self.sums[index];
            index &= index - 1;
        }

        (count, sum)
    }

    fn kth(&self, target: i32) -> usize {
        let mut index = 0usize;
        let mut accumulated = 0i32;
        let mut step = 1usize;

        while (step << 1) < self.counts.len() {
            step <<= 1;
        }

        while step > 0 {
            let next = index + step;

            if next < self.counts.len() && accumulated + self.counts[next] < target {
                index = next;
                accumulated += self.counts[next];
            }

            step >>= 1;
        }

        index + 1
    }
}
