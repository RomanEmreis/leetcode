unsafe public ref struct Queue {
  private int* _queue;
	
  private int _head = 0;
  private int _tail = 0;
  private int _size;
	
  public Queue(int size = 10) {
    _size = size;
    _queue = stackalloc int[size];
  }

  public void Enqueue(int element) {
    if (_tail > _size) throw new Exception("Queue Overflow");

    _queue[_tail] = element;
    if (_tail == _size) _tail = 1;
    else _tail++;
  }
	
  public int Dequeue() {
    int element = _queue[_head];
    if (_head == _size) _head = 1;
    else _head++;

   return element;
  }
}
