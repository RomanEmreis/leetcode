/*
  3508. Implement Router
  
  Design a data structure that can efficiently manage data packets in a network router. Each data packet consists of the following attributes:
      source: A unique identifier for the machine that generated the packet.
      destination: A unique identifier for the target machine.
      timestamp: The time at which the packet arrived at the router.
  
  Implement the Router class:
  Router(int memoryLimit): Initializes the Router object with a fixed memory limit.
      memoryLimit is the maximum number of packets the router can store at any given time.
      If adding a new packet would exceed this limit, the oldest packet must be removed to free up space.
  
  bool addPacket(int source, int destination, int timestamp): Adds a packet with the given attributes to the router.
      A packet is considered a duplicate if another packet with the same source, destination, and timestamp already exists in the router.
      Return true if the packet is successfully added (i.e., it is not a duplicate); otherwise return false.
  
  int[] forwardPacket(): Forwards the next packet in FIFO (First In First Out) order.
      Remove the packet from storage.
      Return the packet as an array [source, destination, timestamp].
      If there are no packets to forward, return an empty array.
  
  int getCount(int destination, int startTime, int endTime):
      Returns the number of packets currently stored in the router (i.e., not yet forwarded) that have the specified destination and have timestamps in the inclusive range [startTime, endTime].
  
  Note that queries for addPacket will be made in increasing order of timestamp.
  
  Example 1:
  Input:
  ["Router", "addPacket", "addPacket", "addPacket", "addPacket", "addPacket", "forwardPacket", "addPacket", "getCount"]
  [[3], [1, 4, 90], [2, 5, 90], [1, 4, 90], [3, 5, 95], [4, 5, 105], [], [5, 2, 110], [5, 100, 110]]
  Output:
  [null, true, true, false, true, true, [2, 5, 90], true, 1]
  Explanation
  Router router = new Router(3); // Initialize Router with memoryLimit of 3.
  router.addPacket(1, 4, 90); // Packet is added. Return True.
  router.addPacket(2, 5, 90); // Packet is added. Return True.
  router.addPacket(1, 4, 90); // This is a duplicate packet. Return False.
  router.addPacket(3, 5, 95); // Packet is added. Return True
  router.addPacket(4, 5, 105); // Packet is added, [1, 4, 90] is removed as number of packets exceeds memoryLimit. Return True.
  router.forwardPacket(); // Return [2, 5, 90] and remove it from router.
  router.addPacket(5, 2, 110); // Packet is added. Return True.
  router.getCount(5, 100, 110); // The only packet with destination 5 and timestamp in the inclusive range [100, 110] is [4, 5, 105]. Return 1.
  
  Example 2:
  Input:
  ["Router", "addPacket", "forwardPacket", "forwardPacket"]
  [[2], [7, 4, 90], [], []]
  
  Output:
  [null, true, [7, 4, 90], []]
  Explanation
  Router router = new Router(2); // Initialize Router with memoryLimit of 2.
  router.addPacket(7, 4, 90); // Return True.
  router.forwardPacket(); // Return [7, 4, 90].
  router.forwardPacket(); // There are no packets left, return [].
*/
use std::collections::{VecDeque, HashSet, HashMap};

struct Router {
    data: VecDeque<(i32, i32, i32)>,
    uniq: HashSet<(i32, i32, i32)>,
    bydest: HashMap<i32, VecDeque<i32>>,
    n: usize,
}

impl Router {
    fn new(memoryLimit: i32) -> Self {
        Self { 
            data: VecDeque::with_capacity(memoryLimit as usize),
            uniq: HashSet::new(),
            n: memoryLimit as usize,
            bydest: HashMap::new(),
        }
    }
    
    #[inline]
    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let res = if !self.uniq.insert((source, destination, timestamp)) {
            false
        } else {
            if self.data.len() == self.n {
                self.pop_packet();
            }
            self.data.push_back((source, destination, timestamp));
            self.bydest.entry(destination).or_insert_with(|| VecDeque::new()).push_back(timestamp);
            true
        };
        res
    }

    #[inline]
    fn pop_packet(&mut self) -> Option<(i32, i32, i32)> {
        if let Some((source, destination, timestamp)) = self.data.pop_front() {
            self.uniq.remove(&(source, destination, timestamp));
            self.bydest.entry(destination).and_modify(|x| { x.pop_front(); });
            Some((source, destination, timestamp))
        } else {
            None
        }
    }
    
    #[inline]
    fn forward_packet(&mut self) -> Vec<i32> {
        let res = if let Some((source, destination, timestamp)) = self.pop_packet() {
            vec![source, destination, timestamp]
        } else {
            Vec::new()
        };
        res
    }
    
    #[inline]
    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(d) = self.bydest.get(&destination) {
            let pp1 = d.partition_point(|&x| x < start_time);
            let pp2 = d.partition_point(|&x| x <= end_time);
            (pp2 - pp1) as i32
        } else {
            0
        }
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
