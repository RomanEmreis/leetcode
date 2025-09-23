/*
  165. Compare Version Numbers
  
  Given two version strings, version1 and version2, compare them. A version string consists of revisions separated by dots '.'. The value of the revision is its integer conversion ignoring leading zeros.
  To compare version strings, compare their revision values in left-to-right order. If one of the version strings has fewer revisions, treat the missing revision values as 0.
  Return the following:
    If version1 < version2, return -1.
    If version1 > version2, return 1.
  
  Otherwise, return 0.
  
  Example 1:
  Input: version1 = "1.2", version2 = "1.10"
  Output: -1
  Explanation:
  version1's second revision is "2" and version2's second revision is "10": 2 < 10, so version1 < version2.
  
  Example 2:
  Input: version1 = "1.01", version2 = "1.001"
  Output: 0
  Explanation:
  Ignoring leading zeroes, both "01" and "001" represent the same integer "1".
  
  Example 3:
  Input: version1 = "1.0", version2 = "1.0.0.0"
  Output: 0
  Explanation:
  version1 has less revisions, which means every missing revision are treated as "0".
*/
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let n = version1.len();
        let m = version2.len();

        let b1 = version1.as_bytes();
        let b2 = version2.as_bytes();

        let mut i = 0;
        let mut j = 0;
        
        while i < n || j < m {
            let mut v1 = 0;
            while i < n && b1[i] != b'.' {
                v1 = v1 * 10 + (b1[i] - b'0') as i32;
                i += 1;
            }
            
            let mut v2 = 0;
            while j < m && b2[j] != b'.' {
                v2 = v2 * 10 + (b2[j] - b'0') as i32;
                j += 1;
            }

            if v1 > v2 {
                return 1;
            } else if v1 < v2 {
                return -1;
            }

            i += 1;
            j += 1;
        }
        
        0        
    }
}
