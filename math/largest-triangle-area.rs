/*
  812. Largest Triangle Area
  
  Given an array of points on the X-Y plane points where points[i] = [xi, yi], return the area of the largest triangle that can be formed by any three different points. Answers within 10-5 of the actual answer will be accepted.
  
  Example 1:
  Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
  Output: 2.00000
  Explanation: The five points are shown in the above figure. The red triangle is the largest.
  
  Example 2:
  Input: points = [[1,0],[0,0],[0,1]]
  Output: 0.50000
*/
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut area: f64 = 0.0;
        for p1 in points.iter() {
            let (x1, y1) = (p1[0], p1[1]);
            for p2 in points.iter() {
                let (x2, y2) = (p2[0], p2[1]);
                for p3 in points.iter() {
                    area = area.max(0.5 * (((x2 - x1) * (p3[1] - y1)) - ((p3[0] - x1) * (y2 - y1))) as f64);
                }
            }   
        }
        area
    }
}
