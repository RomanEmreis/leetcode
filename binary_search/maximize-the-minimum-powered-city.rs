/*
  2528. Maximize the Minimum Powered City
  
  You are given a 0-indexed integer array stations of length n, where stations[i] represents the number of power stations in the ith city.
  Each power station can provide power to every city in a fixed range. In other words, if the range is denoted by r, then a power station at city i can provide power to all cities j such that |i - j| <= r and 0 <= i, j <= n - 1.
      Note that |x| denotes absolute value. For example, |7 - 5| = 2 and |3 - 10| = 7.
  
  The power of a city is the total number of power stations it is being provided power from.
  The government has sanctioned building k more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.
  
  Given the two integers r and k, return the maximum possible minimum power of a city, if the additional power stations are built optimally.
  
  Note that you can build the k power stations in multiple cities.
  
  Example 1:
  Input: stations = [1,2,4,5,0], r = 1, k = 2
  Output: 5
  Explanation: 
  One of the optimal ways is to install both the power stations at city 1. 
  So stations will become [1,4,4,5,0].
  - City 0 is provided by 1 + 4 = 5 power stations.
  - City 1 is provided by 1 + 4 + 4 = 9 power stations.
  - City 2 is provided by 4 + 4 + 5 = 13 power stations.
  - City 3 is provided by 5 + 4 = 9 power stations.
  - City 4 is provided by 5 + 0 = 5 power stations.
  So the minimum power of a city is 5.
  Since it is not possible to obtain a larger power, we return 5.
  
  Example 2:
  Input: stations = [4,4,4,4], r = 0, k = 3
  Output: 4
  Explanation: 
  It can be proved that we cannot make the minimum power of a city greater than 4.
*/
impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let k = k as i64;
        let r = r as usize;
        let stations: Vec<i64> = stations.into_iter().map(|s| s as i64).collect();
        let mut left = 0;
        let mut right = k + 1;
        stations.iter().for_each(|s| {
            left = left.min(*s);
            right += s;
        });
        while left < right {
            let mid = (right + left) >> 1;
            if check(stations.clone(), r, k, mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left - 1
    }
}

fn check(mut stations: Vec<i64>, r: usize, mut k: i64, mid: i64) -> bool {
    let n = stations.len();
    let mut p: i64 = stations[..r].iter().sum();
    for i in 0..n {
        if i + r < n {
            p += stations[i + r];
        }
        if p < mid {
            let req = mid - p;
            if req > k {
                return false;
            }

            stations[std::cmp::min(n - 1, i + r)] += req;
            p += req;
            k -= req;
        }
        if i >= r && i - r >= 0 {
            p -= stations[i - r];
        }
    }
    true
}
