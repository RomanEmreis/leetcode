/*
  You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats where each boat can carry a maximum weight of limit.
  Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.

  Return the minimum number of boats to carry every given person.
*/
public class Solution {
    public int NumRescueBoats(int[] people, int limit) {
        if (people.Length == 1) return 1;

        Array.Sort(people);

        int l = 0, r = people.Length - 1, result = 0;
        while (l <= r) {
            if (people[l] + people[r] <= limit) ++l;
            --r;
            ++result;
        }

        return result;
    }
}
