/*
  3606. Coupon Code Validator
  
  You are given three arrays of length n that describe the properties of n coupons: code, businessLine, and isActive. The ith coupon has:
      code[i]: a string representing the coupon identifier.
      businessLine[i]: a string denoting the business category of the coupon.
      isActive[i]: a boolean indicating whether the coupon is currently active.
  
  A coupon is considered valid if all of the following conditions hold:
      code[i] is non-empty and consists only of alphanumeric characters (a-z, A-Z, 0-9) and underscores (_).
      businessLine[i] is one of the following four categories: "electronics", "grocery", "pharmacy", "restaurant".
      isActive[i] is true.
  
  Return an array of the codes of all valid coupons, sorted first by their businessLine in the order: "electronics", "grocery", "pharmacy", "restaurant", 
  and then by code in lexicographical (ascending) order within each category.
  
  Example 1:
  Input: code = ["SAVE20","","PHARMA5","SAVE@20"], businessLine = ["restaurant","grocery","pharmacy","restaurant"], isActive = [true,true,true,true]
  Output: ["PHARMA5","SAVE20"]
  Explanation:
      First coupon is valid.
      Second coupon has empty code (invalid).
      Third coupon is valid.
      Fourth coupon has special character @ (invalid).
  
  Example 2:
  Input: code = ["GROCERY15","ELECTRONICS_50","DISCOUNT10"], businessLine = ["grocery","electronics","invalid"], isActive = [false,true,true]
  Output: ["ELECTRONICS_50"]
  Explanation:
      First coupon is inactive (invalid).
      Second coupon is valid.
      Third coupon has invalid business line (invalid).
*/
use std::collections::BTreeMap;

impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        let mut key_buf = String::with_capacity(105);

        for i in 0..code.len() {
            if !is_active[i] {
                continue;
            }

            let c = &code[i];
            if !is_valid_code(c) {
                continue;
            }

            if let Some(cat) = get_category(&business_line[i]) {
                key_buf.clear();
                key_buf.push((b'0' + cat) as char);
                key_buf.push_str(c);
                key_buf.push_str(&i.to_string());

                map.insert(key_buf.clone(), c.clone());
            }
        }

        map.into_values().collect()
    }
}

#[inline]
fn is_valid_code(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

#[inline]
fn get_category(business_line: &str) -> Option<u8> {
    match business_line {
        "electronics" => Some(1),
        "grocery" => Some(2),
        "pharmacy" => Some(3),
        "restaurant" => Some(4),
        _ => None,
    }
}
