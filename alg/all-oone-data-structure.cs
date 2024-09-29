/*
  Design a data structure to store the strings' count with the ability to return the strings with minimum and maximum counts.
  
  Implement the AllOne class:
  AllOne() Initializes the object of the data structure.
  inc(String key) Increments the count of the string key by 1. If key does not exist in the data structure, insert it with count 1.
  dec(String key) Decrements the count of the string key by 1. If the count of key is 0 after the decrement, remove it from the data structure. It is guaranteed that key exists in the data structure before the decrement.
  getMaxKey() Returns one of the keys with the maximal count. If no element exists, return an empty string "".
  getMinKey() Returns one of the keys with the minimum count. If no element exists, return an empty string "".
  Note that each function must run in O(1) average time complexity.
  
  Example 1:
    Input
    ["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
    [[], ["hello"], ["hello"], [], [], ["leet"], [], []]
    Output
    [null, null, null, "hello", "hello", null, "hello", "leet"]
    
    Explanation
    AllOne allOne = new AllOne();
    allOne.inc("hello");
    allOne.inc("hello");
    allOne.getMaxKey(); // return "hello"
    allOne.getMinKey(); // return "hello"
    allOne.inc("leet");
    allOne.getMaxKey(); // return "hello"
    allOne.getMinKey(); // return "leet"
*/
public class AllOne {
    private Dictionary<string, int> m = new Dictionary<string, int>();
    private KeyValuePair<string, int> _min = new KeyValuePair<string, int>("", int.MaxValue);
    private KeyValuePair<string, int> _max = new KeyValuePair<string, int>("", int.MinValue);

    public AllOne() {
    }

    private void UpdateMin() {
        _min = new KeyValuePair<string, int>("", int.MaxValue);
        foreach (var nd in m) {
            if (_min.Value > nd.Value) {
                _min = new KeyValuePair<string, int>(nd.Key, nd.Value);
            }
        }
    }

    private void UpdateMax() {
        _max = new KeyValuePair<string, int>("", int.MinValue);
        foreach (var nd in m) {
            if (_max.Value < nd.Value) {
                _max = new KeyValuePair<string, int>(nd.Key, nd.Value);
            }
        }
    }

    public void Inc(string key) {
        if (!m.ContainsKey(key)) {
            m[key] = 0;
        }
        m[key]++;
        if (m[key] > _max.Value) {
            _max = new KeyValuePair<string, int>(key, m[key]);
        }
       
        if (key == _min.Key) {
            UpdateMin();
        }
        if (_min.Value > m[key]) {
            _min = new KeyValuePair<string, int>(key, m[key]);
        }
    }

    public void Dec(string key) {
        m[key]--;
        if (m[key] == 0) {
            m.Remove(key);
        }
        _min = _max = new KeyValuePair<string, int>("", 0);
    }

    public string GetMaxKey() {
        if (!string.IsNullOrEmpty(_max.Key)) {
            return _max.Key;
        }
        UpdateMax();
        return _max.Key;
    }

    public string GetMinKey() {
        if (!string.IsNullOrEmpty(_min.Key)) {
            return _min.Key;
        }
        UpdateMin();
        return _min.Key;
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * AllOne obj = new AllOne();
 * obj.Inc(key);
 * obj.Dec(key);
 * string param_3 = obj.GetMaxKey();
 * string param_4 = obj.GetMinKey();
 */
