/*
  You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a double booking.
  A double booking happens when two events have some non-empty intersection (i.e., some moment is common to both events.).
  The event can be represented as a pair of integers start and end that represents a booking on the half-open interval [start, end), the range of real numbers x such that start <= x < end.
  
  Implement the MyCalendar class:
  MyCalendar() Initializes the calendar object.
  boolean book(int start, int end) Returns true if the event can be added to the calendar successfully without causing a double booking. Otherwise, return false and do not add the event to the calendar.
   
  
  Example 1:
    Input
    ["MyCalendar", "book", "book", "book"]
    [[], [10, 20], [15, 25], [20, 30]]
    Output
    [null, true, false, true]
*/
struct Segment {
    int start;
    int end;
    Segment* left;
    Segment* right;
};

class MyCalendar {
private:
    Segment* root;
public:
    MyCalendar() {
        root = nullptr;
    }
    
    bool book(int start, int end) {
        if (root == nullptr) {
            root = new Segment {
                .start = start,
                .end = end
            };
            return true;
        }
        return insert(root, start, end);
    }

    bool insert(Segment* node, int start, int end) {
        if (end <= node->start) {
            if (!node->left) {
                node->left = new Segment {
                    .start = start,
                    .end = end
                };
                return true;
            }
            return insert(node->left, start, end);
        } else if (start >= node->end) {
            if (!node->right) {
                node->right = new Segment {
                    .start = start,
                    .end = end
                };
                return true;
            }
            return insert(node->right, start, end);
        }
        return false;
    }
};

/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar* obj = new MyCalendar();
 * bool param_1 = obj->book(start,end);
 */
