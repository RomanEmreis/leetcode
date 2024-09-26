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
    start: i32,
    end: i32,
    left: Option<Box<Segment>>,
    right: Option<Box<Segment>>
}

impl Segment {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end, left: None, right: None }
    }
}

struct MyCalendar {
    root: Option<Box<Segment>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self { root: None }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some(ref mut root) = self.root {
            Self::insert(root, start, end)
        } else {
            self.root = Some(Box::new(Segment::new(start, end)));
            true
        }
    }

    fn insert(node: &mut Box<Segment>, start: i32, end: i32) -> bool {
        if end <= node.start {
            if let Some(ref mut left) = node.left {
                return Self::insert(left, start, end);
            } else {
                node.left = Some(Box::new(Segment::new(start, end)));
                return true;
            }
        } else if start >= node.end {
            if let Some(ref mut right) = node.right {
                return Self::insert(right, start, end);
            } else {
                node.right = Some(Box::new(Segment::new(start, end)));
                return true;
            }
        } else {
            false
        }
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
