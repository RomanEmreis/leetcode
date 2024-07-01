/*
  Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.
 
  Example:
    Input: arr = [2,6,4,1]
    Output: false
    Explanation: There are no three consecutive odds.
*/
func threeConsecutiveOdds(arr []int) bool {
    if len(arr) < 3 {
        return false;
    }

    i := 0;
    for _, n := range arr {
        if n % 2 == 1 {
            i++;
        } else {
            i = 0;
        }

        if i == 3 {
            return true;
        }
    }

    return false
}
