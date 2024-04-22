#include <iostream>
#include <stack>

using namespace std;

class MyQueue {
private:
    stack<int> _input, _output;

    void swap() {
        for (; !_input.empty(); _input.pop()) _output.push(_input.top());
    }

public:
    MyQueue() {
    }
    
    void push(int x) {
        _input.push(x);
    }
    
    int pop() {
        if (_output.empty()) swap();
        int x = _output.top();
        _output.pop();
        return x;
    }
    
    int peek() {
        if (_output.empty()) swap();
        return _output.top();
    }
    
    bool empty() {
        return _input.empty() && _output.empty();
    }
};

int main()
{
    MyQueue* q = new MyQueue();

    q->push(1);
    q->push(2);

    int x = q->pop();
    int y = q->pop();
}
