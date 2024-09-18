/*
  Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.
  
  Since the result may be very large, so you need to return a string instead of an integer.
  
  Example 1:
    Input: nums = [10,2]
    Output: "210"
  
  Example 2:
    Input: nums = [3,30,34,5,9]
    Output: "9534330"
*/
type ByLargestNumber []string

func (a ByLargestNumber) Len() int {
	return len(a);
}

func (a ByLargestNumber) Swap(i, j int) {
	a[i], a[j] = a[j], a[i];
}

func (a ByLargestNumber) Less(i, j int) bool {
	return a[i]+a[j] > a[j]+a[i];
}

func largestNumber(nums []int) string {
    strNums := make([]string, len(nums));
	for i, num := range nums {
		strNums[i] = strconv.Itoa(num);
	}
	sort.Sort(ByLargestNumber(strNums));

	if strNums[0] == "0" {
		return "0";
	}

	return strings.Join(strNums, "");
}
