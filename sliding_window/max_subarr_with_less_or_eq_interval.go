/*
  Given an array of integers nums and an integer limit, 
  return the size of the longest non-empty subarray such that the absolute difference between any two elements of this subarray is less than or equal to limit.

  Example:
    Input: nums = [8,2,4,7], limit = 4
    Output: 2 
    Explanation: All subarrays are: 
    [8] with maximum absolute diff |8-8| = 0 <= 4.
    [8,2] with maximum absolute diff |8-2| = 6 > 4. 
    [8,2,4] with maximum absolute diff |8-2| = 6 > 4.
    [8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
    [2] with maximum absolute diff |2-2| = 0 <= 4.
    [2,4] with maximum absolute diff |2-4| = 2 <= 4.
    [2,4,7] with maximum absolute diff |2-7| = 5 > 4.
    [4] with maximum absolute diff |4-4| = 0 <= 4.
    [4,7] with maximum absolute diff |4-7| = 3 <= 4.
    [7] with maximum absolute diff |7-7| = 0 <= 4. 
  Therefore, the size of the longest subarray is 2.
*/
func longestSubarray(nums []int, limit int) int {
    n := len(nums);
    if n == 1 {
        return 1;
    }

	minQ := list.New();
	maxQ := list.New();

	i, result := 0, 0;
	for j := 0; j < n; j++ {
		for minQ.Len() != 0 && nums[minQ.Back().Value.(int)] > nums[j] {
			minQ.Remove(minQ.Back());
		}
		for maxQ.Len() != 0 && nums[maxQ.Back().Value.(int)] < nums[j] {
			maxQ.Remove(maxQ.Back());
		}

		minQ.PushBack(j);
		maxQ.PushBack(j);

		for nums[maxQ.Front().Value.(int)]-nums[minQ.Front().Value.(int)] > limit {
			if maxQ.Front().Value.(int) == i {
				maxQ.Remove(maxQ.Front());
			}
			if minQ.Front().Value.(int) == i {
				minQ.Remove(minQ.Front());
			}
			i++;
		}

		result = max(result, j - i + 1);
	}
    
	return result;
}

func max(a, b int) int {
	if a > b {
		return a;
	}

	return b;
}
