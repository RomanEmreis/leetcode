/*
  Convert a non-negative integer num to its English words representation.
  
  Example:
    Input: num = 123
    Output: "One Hundred Twenty Three"
*/
func numberToWords(num int) string {
	if num == 0 {
		return "Zero";
	}

	singles := []string{"", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"};
	teens := []string{"Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"};
	tens := []string{"", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"};
	thousands := []string{"", "Thousand", "Million", "Billion"};

	var parts []string;
	for i := 0; num > 0; i++ {
		if num%1000 != 0 {
			parts = append([]string{trim(convert(num%1000, singles, teens, tens)) + " " + thousands[i]}, parts...);
		}
		num /= 1000;
	}

	return trim(strings.Join(parts, " "));
}

func convert(n int, singles, teens, tens []string) string {
	var sb strings.Builder;
	if n >= 100 {
	    sb.WriteString(singles[n/100] + " Hundred ");
		n = n % 100;
	}
	if n >= 20 {
		sb.WriteString(tens[n/10] + " ");
		n = n % 10;
	}
	if n >= 10 {
		sb.WriteString(teens[n-10]);
	} else if n > 0 {
		sb.WriteString(singles[n]);
	}
	return sb.String();
}

func trim(s string) string {
	return strings.TrimSpace(s);
}
