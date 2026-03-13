/*
  3296. Minimum Number of Seconds to Make Mountain Height Zero
  
  You are given an integer mountainHeight denoting the height of a mountain.
  You are also given an integer array workerTimes representing the work time of workers in seconds.
  
  The workers work simultaneously to reduce the height of the mountain. For worker i:
      To decrease the mountain's height by x, it takes workerTimes[i] + workerTimes[i] * 2 + ... + workerTimes[i] * x seconds. For example:
          To reduce the height of the mountain by 1, it takes workerTimes[i] seconds.
          To reduce the height of the mountain by 2, it takes workerTimes[i] + workerTimes[i] * 2 seconds, and so on.
  
  Return an integer representing the minimum number of seconds required for the workers to make the height of the mountain 0.
  
  Example 1:
  Input: mountainHeight = 4, workerTimes = [2,1,1]
  Output: 3
  Explanation:
  One way the height of the mountain can be reduced to 0 is:
      Worker 0 reduces the height by 1, taking workerTimes[0] = 2 seconds.
      Worker 1 reduces the height by 2, taking workerTimes[1] + workerTimes[1] * 2 = 3 seconds.
      Worker 2 reduces the height by 1, taking workerTimes[2] = 1 second.
  
  Since they work simultaneously, the minimum time needed is max(2, 3, 1) = 3 seconds.
  
  Example 2:
  Input: mountainHeight = 10, workerTimes = [3,2,2,4]
  Output: 12
  Explanation:
      Worker 0 reduces the height by 2, taking workerTimes[0] + workerTimes[0] * 2 = 9 seconds.
      Worker 1 reduces the height by 3, taking workerTimes[1] + workerTimes[1] * 2 + workerTimes[1] * 3 = 12 seconds.
      Worker 2 reduces the height by 3, taking workerTimes[2] + workerTimes[2] * 2 + workerTimes[2] * 3 = 12 seconds.
      Worker 3 reduces the height by 2, taking workerTimes[3] + workerTimes[3] * 2 = 12 seconds.
  
  The number of seconds needed is max(9, 12, 12, 12) = 12 seconds.
  
  Example 3:
  Input: mountainHeight = 5, workerTimes = [1]
  Output: 15
  Explanation:
  There is only one worker in this example, so the answer is workerTimes[0] + workerTimes[0] * 2 + workerTimes[0] * 3 + workerTimes[0] * 4 + workerTimes[0] * 5 = 15.
*/
impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mountain_height = mountain_height as i64;
        let min_worker = *worker_times.iter().min().unwrap() as i64;

        let mut l = 1i64;
        let mut r = min_worker * mountain_height * (mountain_height + 1) / 2;

        while l < r {
            let m = (l + r) / 2;
            if get_reduced(&worker_times, m) < mountain_height {
                l = m + 1;
            } else {
                r = m;
            }
        }
        
        l
    }
}

#[inline(always)]
fn get_reduced(worker_times: &[i32], m: i64) -> i64 {
    worker_times
        .iter()
        .fold(0, |acc, &time| acc + ((-1.0 + ((1.0 + 8.0 * (m as f64) / (time as f64)).sqrt())) / 2.0) as i64) 
}
