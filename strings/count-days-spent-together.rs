/*
  2409. Count Days Spent Together
  
  Alice and Bob are traveling to Rome for separate business meetings.
  You are given 4 strings arriveAlice, leaveAlice, arriveBob, and leaveBob. Alice will be in the city from the dates arriveAlice to leaveAlice (inclusive), 
  while Bob will be in the city from the dates arriveBob to leaveBob (inclusive). 
  Each will be a 5-character string in the format "MM-DD", corresponding to the month and day of the date.
  
  Return the total number of days that Alice and Bob are in Rome together.
  
  You can assume that all dates occur in the same calendar year, which is not a leap year. 
  Note that the number of days per month can be represented as: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31].
  
  Example 1:
  Input: arriveAlice = "08-15", leaveAlice = "08-18", arriveBob = "08-16", leaveBob = "08-19"
  Output: 3
  Explanation: Alice will be in Rome from August 15 to August 18. Bob will be in Rome from August 16 to August 19.
  They are both in Rome together on August 16th, 17th, and 18th, so the answer is 3.
  
  Example 2:
  Input: arriveAlice = "10-01", leaveAlice = "10-31", arriveBob = "11-01", leaveBob = "12-31"
  Output: 0
  Explanation: There is no day when Alice and Bob are in Rome together, so we return 0.
*/
impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        fn day_of_year(date: &str) -> i32 {
            const DAYS_BEFORE_MONTH: [i32; 12] = [
                0, 31, 59, 90, 120, 151,
                181, 212, 243, 273, 304, 334,
            ];

            let (month, day) = date.split_once('-').unwrap();
            let month = month.parse::<usize>().unwrap();
            let day = day.parse::<i32>().unwrap();

            DAYS_BEFORE_MONTH[month - 1] + day
        }

        let start = day_of_year(&arrive_alice)
            .max(day_of_year(&arrive_bob));

        let end = day_of_year(&leave_alice)
            .min(day_of_year(&leave_bob));

        (end - start + 1).max(0)
    }
}
