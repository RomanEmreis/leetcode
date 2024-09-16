/*
  Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.
  
  Example 1:
    Input: timePoints = ["23:59","00:00"]
    Output: 1

  Example 2:
    Input: timePoints = ["00:00","23:59","00:00"]
    Output: 0
*/
func findMinDifference(timePoints []string) int {
    times := make([]bool, 1440);

    for i := 0; i < len(timePoints); i++ {
        minutes := toMinutes(timePoints[i]);
        if times[minutes] {
            return 0;
        }
        times[minutes] = true;
    }

    prev, first, next := -1, -1, 0;
    min := math.MaxInt32;

    for i := 0; i < 1440; i++ {
        if times[i] {
            if prev < 0 {
                first = i;
            } else {
                next = i;
                if min > next - prev {
                    min = next - prev;
                }
            }
            prev = i;
        }
    }

    if min < first - prev + 1440 {
        return min
    } else {
        return first - prev + 1440;
    }
}

func toMinutes(time string) int {
    hour, _ := strconv.Atoi(time[0:2]);
    minute, _ := strconv.Atoi(time[3:]);

    return hour*60 + minute
}
