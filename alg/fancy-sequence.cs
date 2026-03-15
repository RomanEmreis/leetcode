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
public sealed class Fancy {
    private const long Mod = 1_000_000_007;

    private readonly List<long> raw;
    
    private long mul = 1;
    private long add = 0;

    public Fancy() {
        raw = [];
    }
    
    public void Append(int val) {
        long r = (val - add % Mod + Mod) % Mod * ModInverse(mul) % Mod;
        raw.Add(r);
    }
    
    public void AddAll(int inc) {
        add = (add + inc) % Mod;
    }
    
    public void MultAll(int m) {
        mul = (mul * m) % Mod;
        add = (add * m) % Mod;
    }
    
    public int GetIndex(int idx) {
        if (idx < raw.Count)
            return (int) ((mul * raw[idx] + add) % Mod);

        return -1;
    }

    private static long ModInverse(long val) => ModPow(val, Mod - 2, Mod);

    private static long ModPow(long val, long exp, long mod) {
        long res = 1;

        val %= mod;
        while (exp > 0) {
            if ((exp & 1) == 1)
                res = res * val % mod;
            
            val = val * val % mod;
            exp >>= 1;
        }

        return res;
    }
}

/**
 * Your Fancy object will be instantiated and called as such:
 * Fancy obj = new Fancy();
 * obj.Append(val);
 * obj.AddAll(inc);
 * obj.MultAll(m);
 * int param_4 = obj.GetIndex(idx);
 */
