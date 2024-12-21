/*
  1792. Maximum Average Pass Ratio
  
  There is a school that has classes of students and each class will be having a final exam. You are given a 2D integer array classes, where classes[i] = [passi, totali]. 
  You know beforehand that in the ith class, there are totali total students, but only passi number of students will pass the exam.
  You are also given an integer extraStudents. There are another extraStudents brilliant students that are guaranteed to pass the exam of any class they are assigned to.
  You want to assign each of the extraStudents students to a class in a way that maximizes the average pass ratio across all the classes.
  The pass ratio of a class is equal to the number of students of the class that will pass the exam divided by the total number of students of the class. The average pass ratio is the sum of pass ratios of all the classes divided by the number of the classes.
  
  Return the maximum possible average pass ratio after assigning the extraStudents students. Answers within 10-5 of the actual answer will be accepted.
  
  Example 1:
  Input: classes = [[1,2],[3,5],[2,2]], extraStudents = 2
  Output: 0.78333
  Explanation: You can assign the two extra students to the first class. The average pass ratio will be equal to (3/4 + 3/5 + 2/2) / 3 = 0.78333.
  
  Example 2:
  Input: classes = [[2,4],[3,9],[4,5],[2,10]], extraStudents = 4
  Output: 0.53485
*/
public class Solution {
    public double MaxAverageRatio(int[][] classes, int extraStudents) {
        double result = 0;
        PriorityQueue<(int, int), double> pq = new();
        foreach (var c in classes) {
            int a = c[0], b = c[1];
            double curr = (double) a / b;
            double next = (double) (a + 1) / (b + 1);

            pq.Enqueue((a, b), curr - next);
            result += curr;
        }
        
        while (extraStudents-- > 0 && pq.TryDequeue(out var c, out _)) {
            var (a, b) = c;

            result -= (double) a / b;

            ++a;
            ++b;

            double curr = (double) a / b;
            double next = (double) (a + 1) / (b + 1);

            double x = curr - next;
            pq.Enqueue((a, b), x);

            result += curr;
        }
     
        return result / classes.Length;
    }
}