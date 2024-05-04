/*
  You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats where each boat can carry a maximum weight of limit.
  Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.

  Return the minimum number of boats to carry every given person.
*/
func numRescueBoats(people []int, limit int) int {
    n := len(people);
    if n == 1 {
        return n;
    }

    sort.Ints(people);

    l, r, result := 0, n - 1, 0;
    for l <= r {
        if people[l] + people[r] <= limit {
            l++;
        }
        r--;
        result++;
    }

    return result;
}
