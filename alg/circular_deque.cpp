#include <iostream>
#include <vector>

using namespace std;

class MyCircularDeque {
private:
    int front, rear, capacity;
    vector<int> deque;

public:
    MyCircularDeque(int k) : deque(k + 1) {
        capacity = deque.capacity();
        front = 0;
        rear = 0;
    }
    
    bool insertFront(int value) {
        if (isFull()) return false;

        front = (front - 1 + capacity) % capacity;
        deque[front] = value;

        return true;
    }
    
    bool insertLast(int value) {
        if (isFull()) return false;

        deque[rear] = value;
        rear = (rear + 1) % capacity;

        return true;
    }
    
    bool deleteFront() {
        if (isEmpty()) return false;

        front = (front + 1) % capacity;

        return true;
    }
    
    bool deleteLast() {
        if (isEmpty()) return false;

        rear = (rear - 1 + capacity) % capacity;

        return true;
    }
    
    int getFront() {
        return isEmpty() ? -1 : deque[front];
    }
    
    int getRear() {
        return isEmpty() ? -1 : deque[(rear - 1 + capacity) % capacity];
    }
    
    bool isEmpty() {
        return front == rear;
    }
    
    bool isFull() {
        return (rear + 1) % capacity == front;
    }
};

int main()
{
    MyCircularDeque* d = new MyCircularDeque(3);

    d->insertLast(1);
    d->insertLast(2);
    d->insertFront(3);

    d->insertFront(4);

    auto x = d->getFront();
    auto y = d->getRear();
}
