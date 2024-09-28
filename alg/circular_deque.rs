struct MyCircularDeque {
    front: usize,
    rear: usize,
    capacity: usize,
    deque: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        let k: usize = k as usize + 1;
        Self {
            front: 0,
            rear: 0,
            capacity: k,
            deque: vec![0; k]
        }
    }
    
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        self.front = (self.front - 1 + self.capacity) % self.capacity;
        self.deque[self.front] = value;

        true
    }
    
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        self.deque[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
        true
    }
    
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.front = (self.front + 1) % self.capacity;
        true
    }
    
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.rear = (self.rear - 1 + self.capacity) % self.capacity;
        true
    }
    
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.deque[self.front]
    }
    
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        let idx = (self.rear - 1 + self.capacity) % self.capacity;
        self.deque[idx]
    }
    
    fn is_empty(&self) -> bool {
        self.front == self.rear
    }
    
    fn is_full(&self) -> bool {
        (self.rear + 1) % self.capacity == self.front
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
