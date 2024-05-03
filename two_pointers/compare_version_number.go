/*
  Given two version numbers, version1 and version2, compare them.

  Version numbers consist of one or more revisions joined by a dot '.'. Each revision consists of digits and may contain leading zeros. 
  Every revision contains at least one character. Revisions are 0-indexed from left to right, with the leftmost revision being revision 0, the next revision being revision 1, and so on. 
  For example 2.5.33 and 0.1 are valid version numbers.

  To compare version numbers, compare their revisions in left-to-right order. Revisions are compared using their integer value ignoring any leading zeros. 
  This means that revisions 1 and 001 are considered equal. If a version number does not specify a revision at an index, then treat the revision as 0. 
  For example, version 1.0 is less than version 1.1 because their revision 0s are the same, but their revision 1s are 0 and 1 respectively, and 0 < 1.

  Return the following:

  If version1 < version2, return -1.
  If version1 > version2, return 1.
  Otherwise, return 0.
*/
func compareVersion(version1 string, version2 string) int {
    const dot, zero = '.', '0';
    n1, n2 := len(version1), len(version2);

    i, j := 0, 0;
    for i < n1 || j < n2 {
        num1, num2 := 0, 0;
        for i < n1 && version1[i] != dot {
            digit, _ := strconv.Atoi(string(version1[i]));
			num1 = num1 * 10 + digit;
			i++;
        }
        for j < n2 && version2[j] != dot {
            digit, _ := strconv.Atoi(string(version2[j]));
            num2 = num2 * 10 + digit;
            j++;
        }

        if num1 < num2 {
            return -1;
        } else if num1 > num2 {
            return 1;
        }

        i++;
        j++;
    }

    return 0;
}
