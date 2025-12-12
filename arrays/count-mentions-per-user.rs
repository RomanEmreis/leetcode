/*
  3433. Count Mentions Per User
  
  You are given an integer numberOfUsers representing the total number of users and an array events of size n x 3.
  
  Each events[i] can be either of the following two types:
      Message Event: ["MESSAGE", "timestampi", "mentions_stringi"]
          This event indicates that a set of users was mentioned in a message at timestampi.
          The mentions_stringi string can contain one of the following tokens:
              id<number>: where <number> is an integer in range [0,numberOfUsers - 1]. There can be multiple ids separated by a single whitespace and may contain duplicates. This can mention even the offline users.
              ALL: mentions all users.
              HERE: mentions all online users.
      Offline Event: ["OFFLINE", "timestampi", "idi"]
          This event indicates that the user idi had become offline at timestampi for 60 time units. The user will automatically be online again at time timestampi + 60.
  
  Return an array mentions where mentions[i] represents the number of mentions the user with id i has across all MESSAGE events.
  
  All users are initially online, and if a user goes offline or comes back online, their status change is processed before handling any message event that occurs at the same timestamp.
  
  Note that a user can be mentioned multiple times in a single message event, and each mention should be counted separately.
  
  Example 1:
  Input: numberOfUsers = 2, events = [["MESSAGE","10","id1 id0"],["OFFLINE","11","0"],["MESSAGE","71","HERE"]]
  Output: [2,2]
  Explanation:
  Initially, all users are online.
  At timestamp 10, id1 and id0 are mentioned. mentions = [1,1]
  At timestamp 11, id0 goes offline.
  At timestamp 71, id0 comes back online and "HERE" is mentioned. mentions = [2,2]
  
  Example 2:
  Input: numberOfUsers = 2, events = [["MESSAGE","10","id1 id0"],["OFFLINE","11","0"],["MESSAGE","12","ALL"]]
  Output: [2,2]
  Explanation:
  Initially, all users are online.
  At timestamp 10, id1 and id0 are mentioned. mentions = [1,1]
  At timestamp 11, id0 goes offline.
  At timestamp 12, "ALL" is mentioned. This includes offline users, so both id0 and id1 are mentioned. mentions = [2,2]
  
  Example 3:
  Input: numberOfUsers = 2, events = [["OFFLINE","10","0"],["MESSAGE","12","HERE"]]
  Output: [0,1]
  Explanation:
  Initially, all users are online.
  At timestamp 10, id0 goes offline.
  At timestamp 12, "HERE" is mentioned. Because id0 is still offline, they will not be mentioned. mentions = [0,1]
*/
impl Solution {
    pub fn count_mentions(number_of_users: i32, mut events: Vec<Vec<String>>) -> Vec<i32> {
        let n = number_of_users as usize;

        let mut mentions = vec![0; n];
        let mut status = vec![0; n];

        events.sort_by(|a, b| {
            let t1 = a[1].parse::<i32>().unwrap();
            let t2 = b[1].parse::<i32>().unwrap();
            if t1 == t2 {
                a[0].as_bytes()[2].cmp(&b[0].as_bytes()[2])
            } else {
                t1.cmp(&t2)
            }
        });

        let mut time = 0;
        
        for e in events.into_iter() {
            time = e[1].parse::<i32>().unwrap();
            for i in 0..n {
                if status[i] != 0 && status[i] <= time {
                    status[i] = 0;
                }
            }

            if e[0] == "MESSAGE" {
                if e[2] == "ALL" {
                    for i in 0..n {
                        mentions[i] += 1;
                    }
                } else if e[2] == "HERE" {
                    for i in 0..n {
                        if status[i] == 0 {
                            mentions[i] += 1;
                        }
                    }
                } else {
                    let users = e[2].split(" ").map(|u| u[2..].parse::<usize>().unwrap());
                    for u in users {
                        mentions[u] += 1;
                    }
                }
            } else {
                let user = e[2].parse::<usize>().unwrap();
                status[user] = time + 60;
            }
        }

        mentions
    }
}
