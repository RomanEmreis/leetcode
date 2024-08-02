/*
  A swap is defined as taking two distinct positions in an array and swapping the values in them.
  A circular array is defined as an array where we consider the first element and the last element to be adjacent.

  Given a binary circular array nums, return the minimum number of swaps required to group all 1's present in the array together at any location.

  Example:
    Input: nums = [0,1,0,1,1,0,0]
    Output: 1
    Explanation: Here are a few of the ways to group all the 1's together:
    [0,0,1,1,1,0,0] using 1 swap.
    [0,1,1,1,0,0,0] using 1 swap.
    [1,1,0,0,0,0,1] using 2 swaps (using the circular property of the array).
    There is no way to group all 1's together with 0 swaps.
    Thus, the minimum number of swaps required is 1.
*/
func minSwaps(nums []int) int {
    total := 0;
    n := len(nums);

    for _, num := range nums {
        if num == 1 {
            total++;
        }
    }

    if total == 0 || total == n {
        return 0;
    }

    maxOnes := 0;
    for i := 0; i < total; i++ {
        if nums[i] == 1 {
            maxOnes++;
        }
    }

    currOnes := maxOnes;
    for i := 1; i < n; i++ {
        if nums[i - 1] == 1 {
            currOnes--;
        }
        if nums[(i + total - 1) % n] == 1 {
            currOnes++;
        }
        if currOnes > maxOnes {
            maxOnes = currOnes;
        }
    }

    return total - maxOnes;
}
