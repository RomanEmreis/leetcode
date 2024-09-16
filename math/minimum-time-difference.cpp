/*
  Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.
  
  Example 1:
    Input: timePoints = ["23:59","00:00"]
    Output: 1

  Example 2:
    Input: timePoints = ["00:00","23:59","00:00"]
    Output: 0
*/
class Solution {
public:
    int findMinDifference(vector<string>& timePoints) {
        vector<bool> times(1440);
        for (string& time : timePoints) {
            int minutes = to_minutes(time);
            if (times[minutes]) return 0;
            times[minutes] = true;
        }

        int prev = -1, first = -1, next = 0;
        int m = INT_MAX;
        for (size_t i = 0; i < 1440; ++i) {
            if (times[i]) {
                if (prev < 0) first = i;
                else {
                    next = i;
                    m = min(m, next - prev);
                }
                prev = i;
            }
        }

        return min(m, first - prev + 1440);
    }
private:
    static int to_minutes(string& time) {
        int hour = stoi(time.substr(0, 2));
        int minute = stoi(time.substr(3, 6));

        return hour * 60 + minute;
    }
};
