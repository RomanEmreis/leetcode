/*
  Given a string n representing an integer, return the closest integer (not including itself), which is a palindrome. If there is a tie, return the smaller one.
  
  The closest is defined as the absolute difference minimized between two integers.
  
  Example 1:
    Input: n = "123"
    Output: "121"
  
  Example 2:
    Input: n = "1"
    Output: "0"
    Explanation: 0 and 2 are the closest palindromes but we return the smallest which is 0.
*/
func nearestPalindromic(n string) string {
	original, _ := strconv.ParseInt(n, 10, 64)
	length := len(n)
	
	var candidates []int64
	
	half := n[:(length+1)/2]
	halfNum, _ := strconv.ParseInt(half, 10, 64)
	
	for i := -1; i <= 1; i++ {
		candidates = append(candidates, createPalindrome(strconv.FormatInt(halfNum+int64(i), 10), length))
	}
	
	candidates = append(candidates, int64(math.Pow(10, float64(length-1))) - 1) // biggest number with one fewer digit
	candidates = append(candidates, int64(math.Pow(10, float64(length))) + 1)  // smallest number with one more digit
	
	var minDiff int64 = math.MaxInt64
	var closest int64 = math.MaxInt64
	
	for _, candidate := range candidates {
		if candidate == original {
			continue
		}
		diff := int64(math.Abs(float64(candidate - original)))
		if diff < minDiff || (diff == minDiff && candidate < closest) {
			closest = candidate
			minDiff = diff
		}
	}
	
	return strconv.FormatInt(closest, 10)
}

func createPalindrome(half string, length int) int64 {
	runes := []rune(half)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}

	secondHalf := string(runes)
	if length%2 == 0 {
		palindrome, _ := strconv.ParseInt(half+secondHalf, 10, 64)
		return palindrome
	} else {
		palindrome, _ := strconv.ParseInt(half+secondHalf[1:], 10, 64)
		return palindrome
	}
}
