/*
  Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.
  
  Example 1:
    Input: timePoints = ["23:59","00:00"]
    Output: 1

  Example 2:
    Input: timePoints = ["00:00","23:59","00:00"]
    Output: 0
*/
public class Solution {
    public int FindMinDifference(IList<string> timePoints) {
        Span<bool> times = stackalloc bool[1440];

        for (int i = 0; i < timePoints.Count; ++i) {
            int minutes = ToMinutes(timePoints[i]);
            if (times[minutes]) return 0;
            times[minutes] = true;
        }

        var (prev, first, next) = (-1, -1, 0);
        int result = int.MaxValue;

        for (int i = 0; i < times.Length; ++i) {
            if (times[i]) {
                if (prev < 0) first = i;
                else {
                    next = i;
                    result = Math.Min(result, next - prev); 
                }
                prev = i;
            }
        }

        return Math.Min(result, first - prev + 1440);
    }

    private static int ToMinutes(string time) {
        int hour = int.Parse(time[0..2]);
        int minute = int.Parse(time[3..]);

        return hour * 60 + minute;
    }
}
