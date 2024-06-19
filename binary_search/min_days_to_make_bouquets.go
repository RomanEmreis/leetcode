/*
  You are given an integer array bloomDay, an integer m and an integer k.
  You want to make m bouquets. To make a bouquet, you need to use k adjacent flowers from the garden.

  The garden consists of n flowers, the ith flower will bloom in the bloomDay[i] and then can be used in exactly one bouquet.

  Return the minimum number of days you need to wait to be able to make m bouquets from the garden. If it is impossible to make m bouquets return -1.

  Example:
    Input: bloomDay = [1,10,3,10,2], m = 3, k = 1
    Output: 3
    Explanation: Let us see what happened in the first three days. x means flower bloomed and _ means flower did not bloom in the garden.
    We need 3 bouquets each should contain 1 flower.
    After day 1: [x, _, _, _, _]   // we can only make one bouquet.
    After day 2: [x, _, _, _, x]   // we can only make two bouquets.
    After day 3: [x, _, x, _, x]   // we can make 3 bouquets. The answer is 3.
*/
func minDays(bloomDay []int, m int, k int) int {
    n := len(bloomDay);
    if n < (m * k) {
        return -1;
    }

    result := -1;
    l, r := 1, -1;
    for i := 0; i < n; i++ {
        r = int(math.Max(float64(r), float64(bloomDay[i])));
    }

    for l <= r {
        mid := l + (r - l) / 2;
        if (canMake(&bloomDay, &mid, &m, &k, &n)) {
            result = mid;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    return result;
}

func canMake(bloomDay *[]int, days *int, m *int, k *int, n *int) bool {
    rk, b := 0, 0;
    for i := 0; i < *n; i++ {
        if (*bloomDay)[i] <= *days {
            rk++;
            if rk == *k {
                b++;
                rk = 0;
            }
        } else {
            rk = 0;
        }
    }
    return b >= *m;
}
