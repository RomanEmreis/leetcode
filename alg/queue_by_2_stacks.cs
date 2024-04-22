public class Queue<T> where T : class {
  private Stack<T> input = new Stack<T>();
  private Stack<T> output = new Stack<T>();

  public void Enqueue(T t) {
    input.Push(t);
  }

  public T Dequeue() {
    if (output.Count == 0) {
      while (input.Count != 0) {
        output.Push(input.Pop());
      }
    }
    return output.Pop();
  }
}
