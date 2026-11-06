/*
  2747. Count Zero Request Servers
  
  You are given an integer n denoting the total number of servers and a 2D 0-indexed integer array logs, 
  where logs[i] = [server_id, time] denotes that the server with id server_id received a request at time time.
  
  You are also given an integer x and a 0-indexed integer array queries.
  
  Return a 0-indexed integer array arr of length queries.length where arr[i] represents the number of servers 
  that did not receive any requests during the time interval [queries[i] - x, queries[i]].
  
  Note that the time intervals are inclusive.
  
  Example 1:
  Input: n = 3, logs = [[1,3],[2,6],[1,5]], x = 5, queries = [10,11]
  Output: [1,2]
  Explanation: 
  For queries[0]: The servers with ids 1 and 2 get requests in the duration of [5, 10]. Hence, only server 3 gets zero requests.
  For queries[1]: Only the server with id 2 gets a request in duration of [6,11]. Hence, the servers with ids 1 and 3 are the only servers that do not receive any requests during that time period.
  
  Example 2:
  Input: n = 3, logs = [[2,4],[2,1],[1,2],[3,1]], x = 2, queries = [3,4]
  Output: [0,1]
  Explanation: 
  For queries[0]: All servers get at least one request in the duration of [1, 3].
  For queries[1]: Only server with id 3 gets no request in the duration [2,4].
*/
impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        fn radix_sort_by_hi(a: &mut Vec<u64>) {
            let mut buf = vec![0u64; a.len()];
            for shift in [32u32, 43] {
                let mut cnt = [0usize; 2049];
                for &v in a.iter() {
                    cnt[((v >> shift) & 2047) as usize + 1] += 1;
                }

                for i in 0..2048 {
                    cnt[i + 1] += cnt[i];
                }

                for &v in a.iter() {
                    let k = ((v >> shift) & 2047) as usize;
                    buf[cnt[k]] = v;
                    cnt[k] += 1;
                }
                
                std::mem::swap(a, &mut buf);
            }
        }

        let mut packed_logs: Vec<u64> = logs
            .into_iter()
            .map(|log| ((log[1] as u64) << 32) | log[0] as u32 as u64)
            .collect();

        radix_sort_by_hi(&mut packed_logs);

        let mut packed_queries: Vec<u64> = queries
            .into_iter()
            .enumerate()
            .map(|(i, t)| ((t as u64) << 32) | i as u32 as u64)
            .collect();

        radix_sort_by_hi(&mut packed_queries);

        let mut res = vec![0; packed_queries.len()];
        let mut cnt = vec![0i32; n as usize + 1];

        let mut active = 0i32;
        let mut left = 0usize;
        let mut right = 0usize;
        let m = packed_logs.len();

        for &q in &packed_queries {
            let query_time = (q >> 32) as i32;
            let idx = q as u32 as usize;

            while right < m {
                let p = packed_logs[right];
                if (p >> 32) as i32 > query_time {
                    break;
                }
                let s = p as u32 as usize;
                if cnt[s] == 0 {
                    active += 1;
                }
                cnt[s] += 1;
                right += 1;
            }

            let earliest = query_time - x;
            while left < right {
                let p = packed_logs[left];
                if (p >> 32) as i32 >= earliest {
                    break;
                }
                let s = p as u32 as usize;
                cnt[s] -= 1;
                if cnt[s] == 0 {
                    active -= 1;
                }
                left += 1;
            }

            res[idx] = n - active;
        }

        res
    }
}
