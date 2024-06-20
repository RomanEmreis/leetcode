/*
  In the universe Earth C-137, Rick discovered a special form of magnetic force between two balls if they are put in his new invented basket. 
  Rick has n empty baskets, the ith basket is at position[i], Morty has m balls and needs to distribute the balls into the baskets such that the minimum magnetic force between any two balls is maximum.
  Rick stated that magnetic force between two different balls at positions x and y is |x - y|.

  Given the integer array position and the integer m. Return the required force.

Example 1:
  Input: position = [1,2,3,4,7], m = 3
  Output: 3
  Explanation: Distributing the 3 balls into baskets 1, 4 and 7 will make the magnetic force between ball pairs [3, 3, 6]. The minimum magnetic force is 3. We cannot achieve a larger minimum magnetic force than 3.
*/
func maxDistance(position []int, m int) int {
    sort.Ints(position);

    n := len(position);
    l, r := 1, (position[n - 1] - position[0]) / (m - 1);
    for l <= r {
        mid := l + (r - l) / 2;
        if canPlace(&position, &mid, &m) {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    return r;
}

func canPlace(position *[]int, force *int, m *int) bool {
    count, last := 1, (*position)[0];
    for i := 1; i < len(*position); i++ {
        if (*position)[i] - last >= *force {
            count++;
            last = (*position)[i];

            if count == *m {
                return true;
            }
        }
    }
    return false;
}
