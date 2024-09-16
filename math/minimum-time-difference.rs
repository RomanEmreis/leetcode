/*
  Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.
  
  Example 1:
    Input: timePoints = ["23:59","00:00"]
    Output: 1

  Example 2:
    Input: timePoints = ["00:00","23:59","00:00"]
    Output: 0
*/
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut times = [false; 1440];

        for time in time_points {
            let minutes = Self::to_minutes(&time) as usize;
            if times[minutes] { return 0; }
            times[minutes] = true;
        }

        let mut prev = -1;
        let mut first = -1;
        let mut next = 0;
        let mut min = i32::MAX;

        for i in 0..1440 {
            if times[i] {
                if prev < 0 { 
                    first = i as i32;
                } else {
                    next = i as i32;
                    min = min.min(next - prev);
                }
                prev = i as i32;
            }
        }

        min.min(first - prev + 1440)
    }

    fn to_minutes(time: &str) -> i32 {
        let hour: i32 = time[0..2].parse().unwrap();
        let minute: i32 = time[3..].parse().unwrap();

        hour * 60 + minute
    }
}
