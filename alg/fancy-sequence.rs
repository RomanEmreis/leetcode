/*
  1622. Fancy Sequence
  
  Write an API that generates fancy sequences using the append, addAll, and multAll operations.
  Implement the Fancy class:
      Fancy() Initializes the object with an empty sequence.
      void append(val) Appends an integer val to the end of the sequence.
      void addAll(inc) Increments all existing values in the sequence by an integer inc.
      void multAll(m) Multiplies all existing values in the sequence by an integer m.
      int getIndex(idx) Gets the current value at index idx (0-indexed) of the sequence modulo 109 + 7. If the index is greater or equal than the length of the sequence, return -1.
  
  Example 1:
  Input
  ["Fancy", "append", "addAll", "append", "multAll", "getIndex", "addAll", "append", "multAll", "getIndex", "getIndex", "getIndex"]
  [[], [2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]]
  Output
  [null, null, null, null, null, 10, null, null, null, 26, 34, 20]
  
  Explanation
  Fancy fancy = new Fancy();
  fancy.append(2);   // fancy sequence: [2]
  fancy.addAll(3);   // fancy sequence: [2+3] -> [5]
  fancy.append(7);   // fancy sequence: [5, 7]
  fancy.multAll(2);  // fancy sequence: [5*2, 7*2] -> [10, 14]
  fancy.getIndex(0); // return 10
  fancy.addAll(3);   // fancy sequence: [10+3, 14+3] -> [13, 17]
  fancy.append(10);  // fancy sequence: [13, 17, 10]
  fancy.multAll(2);  // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]
  fancy.getIndex(0); // return 26
  fancy.getIndex(1); // return 34
  fancy.getIndex(2); // return 20
*/
const MOD: i64 = 1_000_000_007;

struct Fancy {
    raw: Vec<i64>,
    mul: i64,
    add: i64
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {

    fn new() -> Self {
        Self {
            raw: Vec::new(),
            mul: 1,
            add: 0
        }
    }
    
    fn append(&mut self, val: i32) {
        let r = (val as i64 - self.add % MOD + MOD) % MOD * mod_inverse(self.mul) % MOD;
        self.raw.push(r);
    }
    
    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }
    
    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.mul = (self.mul * m) % MOD;
        self.add = (self.add * m) % MOD;
    }
    
    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if self.raw.len() > idx {
            ((self.mul * self.raw[idx] + self.add) % MOD) as i32
        } else {
            -1
        }
    }
}

#[inline(always)]
fn mod_inverse(mut val: i64) -> i64 {
    let mut exp = MOD - 2;
    let mut res = 1;

    val %= MOD;

    while exp > 0 {
        if (exp & 1) == 1 {
            res = res * val % MOD;
        }

        val = val * val % MOD;
        exp >>= 1;
    }

    res
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
