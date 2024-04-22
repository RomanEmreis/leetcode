public class Solution {
    public int CountStudents(int[] students, int[] sandwiches) {
        if (students.Length == 1) return students[0] != sandwiches[0] ? 1 : 0;

        Queue<int> studentsQueue = [];
        Stack<int> sandwichesStack = [];

        for (int i = 0; i < students.Length; ++i) studentsQueue.Enqueue(students[i]);
        for (int i = sandwiches.Length - 1; i >= 0; --i) sandwichesStack.Push(sandwiches[i]);

        int refuse = 0;
        while (studentsQueue.TryDequeue(out int student) && sandwichesStack.TryPeek(out int sandwich))  {
            if (student == sandwich) {
                sandwichesStack.Pop();
                refuse = 0;
            } else {
                studentsQueue.Enqueue(student);
                ++refuse;               
            }

            if (refuse > students.Length) return studentsQueue.Count;
        }

        return studentsQueue.Count;
    }
}
