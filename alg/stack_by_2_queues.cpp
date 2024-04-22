#include <iostream>
#include <queue>

using namespace std;

class MyStack {
private:
    queue<int> _q1;
    queue<int> _q2;

    void swap(queue<int>& q1, queue<int>& q2) {
        for (;!q1.empty(); q1.pop()) {
            int x = q1.front();
            q2.push(x);
        }
    }

public:
    MyStack() {
        
    }
    
    void push(int x) {
        if (_q2.empty()) {
            _q2.push(x);
        } else {
            swap(_q2, _q1);
            _q2.push(x);
            swap(_q1, _q2);
        }
    }
    
    int pop() {
        int x = _q2.front();
        _q2.pop();
        return x;
    }
    
    int top() {
        return _q2.front();
    }
    
    bool empty() {
        return _q2.empty();
    }
};

int main()
{
    MyStack* s = new MyStack();

    s->push(1);
    s->push(2);

    int x = s->pop();
    int y = s->pop();
}
