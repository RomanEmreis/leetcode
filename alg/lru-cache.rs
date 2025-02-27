use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, Rc<RefCell<CacheNode>>>,
    head: Rc<RefCell<CacheNode>>,
    tail: Rc<RefCell<CacheNode>>,
}

struct CacheNode {
    key: i32,
    value: i32,
    prev: Option<Weak<RefCell<CacheNode>>>,
    next: Option<Rc<RefCell<CacheNode>>>,
}

impl CacheNode {
    fn new(key: i32, value: i32) -> Self {
        Self { key, value, prev: None, next: None }
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let head = Rc::new(RefCell::new(CacheNode::new(0, 0)));
        let tail = Rc::new(RefCell::new(CacheNode::new(0, 0)));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));

        Self {
            capacity,
            cache: HashMap::with_capacity(capacity),
            head,
            tail,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key).cloned() {
            self.move_to_head(node.clone());
            return node.borrow().value;
        }
        -1
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.move_to_head(node);
        } else {
            let node = Rc::new(RefCell::new(CacheNode::new(key, value)));
            self.cache.insert(key, node.clone());
            self.add(node.clone());
            
            if self.cache.len() > self.capacity {
                if let Some(removed) = self.pop_tail() {
                    self.cache.remove(&removed.borrow().key);
                }
            }
        }
    }

    fn move_to_head(&self, node: Rc<RefCell<CacheNode>>) {
        self.remove(node.clone());
        self.add(node);
    }

    fn add(&self, node: Rc<RefCell<CacheNode>>) {
        let mut head_next = self.head.borrow_mut().next.take().unwrap();
        node.borrow_mut().prev = Some(Rc::downgrade(&self.head));
        node.borrow_mut().next = Some(head_next.clone());
        head_next.borrow_mut().prev = Some(Rc::downgrade(&node));
        self.head.borrow_mut().next = Some(node);
    }

    fn remove(&self, node: Rc<RefCell<CacheNode>>) {
        let prev = node.borrow_mut().prev.take().unwrap().upgrade().unwrap();
        let next = node.borrow_mut().next.take();
        if let Some(next) = next {
            next.borrow_mut().prev = Some(Rc::downgrade(&prev));
            prev.borrow_mut().next = Some(next);
        } else {
            prev.borrow_mut().next = None;
        }
    }

    fn pop_tail(&self) -> Option<Rc<RefCell<CacheNode>>> {
        let tail_prev = self.tail.borrow_mut().prev.take()?.upgrade()?;
        self.remove(tail_prev.clone());
        Some(tail_prev)
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
