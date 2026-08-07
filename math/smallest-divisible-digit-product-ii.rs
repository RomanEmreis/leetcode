/*
  3348. Smallest Divisible Digit Product II
  
  You are given a string num which represents a positive integer, and an integer t.
  
  A number is called zero-free if none of its digits are 0.
  
  Return a string representing the smallest zero-free number greater than or equal to num such that the product of its digits is divisible by t. If no such number exists, return "-1".
  
  Example 1:
  Input: num = "1234", t = 256
  Output: "1488"
  Explanation:
  The smallest zero-free number that is greater than 1234 and has the product of its digits divisible by 256 is 1488, with the product of its digits equal to 256.
  
  Example 2:
  Input: num = "12355", t = 50
  Output: "12355"
  Explanation:
  12355 is already zero-free and has the product of its digits divisible by 50, with the product of its digits equal to 150.
  
  Example 3:
  Input: num = "11111", t = 26
  Output: "-1"
  Explanation:
  No number greater than 11111 has the product of its digits divisible by 26.
*/
const FACTORS: [[usize; 4]; 10] = [
    [0, 0, 0, 0], // 0
    [0, 0, 0, 0], // 1
    [1, 0, 0, 0], // 2
    [0, 1, 0, 0], // 3
    [2, 0, 0, 0], // 4
    [0, 0, 1, 0], // 5
    [1, 1, 0, 0], // 6
    [0, 0, 0, 1], // 7
    [3, 0, 0, 0], // 8
    [0, 2, 0, 0], // 9
];

impl Solution {
    pub fn smallest_number(num: String, mut t: i64) -> String {
        let mut target = [0usize; 4];

        for (index, prime) in [2i64, 3, 5, 7].into_iter().enumerate() {
            while t % prime == 0 {
                target[index] += 1;
                t /= prime;
            }
        }

        if t != 1 {
            return "-1".to_string();
        }

        let min23 = Self::build_min23(target[0], target[1]);
        let bytes = num.as_bytes();
        let length = bytes.len();

        let first_zero = bytes
            .iter()
            .position(|&byte| byte == b'0')
            .unwrap_or(length);

        let mut prefix = [0usize; 4];

        for &byte in bytes {
            let digit = (byte - b'0') as usize;

            for factor in 0..4 {
                prefix[factor] += FACTORS[digit][factor];
            }
        }

        if first_zero == length
            && (0..4).all(|factor| prefix[factor] >= target[factor])
        {
            return num;
        }

        for position in (0..length).rev() {
            let old_digit = (bytes[position] - b'0') as usize;

            for factor in 0..4 {
                prefix[factor] -= FACTORS[old_digit][factor];
            }

            if position > first_zero {
                continue;
            }

            for new_digit in old_digit + 1..=9 {
                let required = Self::uncovered(target, prefix, new_digit);
                let suffix_length = length - position - 1;

                if Self::min_digits(required, &min23) <= suffix_length {
                    let suffix = Self::build_suffix(
                        required,
                        suffix_length,
                        &min23,
                    );

                    let mut answer = Vec::with_capacity(length);
                    answer.extend_from_slice(&bytes[..position]);
                    answer.push(b'0' + new_digit as u8);
                    answer.extend_from_slice(&suffix);

                    return String::from_utf8(answer).unwrap();
                }
            }
        }

        let required_length = Self::min_digits(target, &min23).max(length + 1);

        String::from_utf8(Self::build_suffix(
            target,
            required_length,
            &min23,
        ))
        .unwrap()
    }

    fn build_min23(max_two: usize, max_three: usize) -> Vec<Vec<usize>> {
        let mut table = vec![vec![0usize; max_three + 1]; max_two + 1];

        for two in 0..=max_two {
            for three in 0..=max_three {
                table[two][three] = (0..=two.min(three))
                    .map(|sixes| {
                        sixes
                            + (two - sixes).div_ceil(3)
                            + (three - sixes).div_ceil(2)
                    })
                    .min()
                    .unwrap();
            }
        }

        table
    }

    fn min_digits(
        required: [usize; 4],
        min23: &[Vec<usize>],
    ) -> usize {
        min23[required[0]][required[1]]
            + required[2]
            + required[3]
    }

    fn uncovered(
        target: [usize; 4],
        prefix: [usize; 4],
        digit: usize,
    ) -> [usize; 4] {
        let mut result = [0usize; 4];

        for factor in 0..4 {
            result[factor] = target[factor].saturating_sub(prefix[factor] + FACTORS[digit][factor]);
        }

        result
    }

    fn build_suffix(
        mut required: [usize; 4],
        length: usize,
        min23: &[Vec<usize>],
    ) -> Vec<u8> {
        let mut result = Vec::with_capacity(length);

        for position in 0..length {
            let remaining_slots = length - position - 1;

            for digit in 1..=9 {
                let next = Self::uncovered(
                    required,
                    [0; 4],
                    digit,
                );

                if Self::min_digits(next, min23) <= remaining_slots {
                    result.push(b'0' + digit as u8);
                    required = next;
                    break;
                }
            }
        }

        result
    }
}
