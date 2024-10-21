/*
  Design and implement a data structure for a Least Frequently Used (LFU) cache.
  
  Implement the LFUCache class:
  
  LFUCache(int capacity) Initializes the object with the capacity of the data structure.
  int get(int key) Gets the value of the key if the key exists in the cache. Otherwise, returns -1.
  void put(int key, int value) Update the value of the key if present, or inserts the key if not already present. When the cache reaches its capacity, it should invalidate and remove the least frequently used key before inserting a new item. For this problem, when there is a tie (i.e., two or more keys with the same frequency), the least recently used key would be invalidated.
  To determine the least frequently used key, a use counter is maintained for each key in the cache. The key with the smallest use counter is the least frequently used key.
  
  When a key is first inserted into the cache, its use counter is set to 1 (due to the put operation). The use counter for a key in the cache is incremented either a get or put operation is called on it.
  
  The functions get and put must each run in O(1) average time complexity.
*/
public sealed class LFUCache {
    private readonly Dictionary<int, Node> cache;
    private readonly Dictionary<int, Nodes> freq;
    private readonly int capacity;

    private int minFreq = 0;

    public LFUCache(int capacity) {
        this.capacity = capacity;
        cache = new(capacity);
        freq = new();
    }
    
    public int Get(int key) {
        if (cache.TryGetValue(key, out Node node)) {
            UpdateNode(node);
            return node.value;
        }
        return -1;
    }
    
    public void Put(int key, int value) {
        if (cache.TryGetValue(key, out Node node)) {
            node.value = value;
            UpdateNode(node);
        } else {
            if (capacity == cache.Count) {
                Nodes frequencies = freq[minFreq];
                Node rem = frequencies.PopTail();
                cache.Remove(rem.key);
            }

            node = new(key, value);
            if (!freq.ContainsKey(1)) freq[1] = new Nodes();

            freq[1].Add(node);
            cache[key] = node;
            minFreq = 1;
        }
    }

    private void UpdateNode(Node node) {
        Nodes frequencies = freq[node.frequency];
        frequencies.Remove(node);
        if (frequencies.count == 0 && minFreq == node.frequency) ++minFreq;

        ++node.frequency;

        if (!freq.ContainsKey(node.frequency)) freq[node.frequency] = new Nodes();

        freq[node.frequency].Add(node);
    }

    sealed class Node(int k = 0, int v = 0) {
        public int key = k;
        public int value = v;
        public int frequency = 1;
        public Node next, prev;
    }

    sealed class Nodes {
        public int count;
        public Node head, tail;

        public Nodes() {
            head = new();
            tail = new();

            head.next = tail;
            tail.prev = head;
        }

        public void Remove(Node node) {
            Node next = node.next;
            Node prev = node.prev;

            prev.next = next;
            next.prev = prev;

            --count;
        }

        public void Add(Node node) {
            node.prev = head;
            node.next = head.next;

            head.next.prev = node;
            head.next = node;

            ++count;
        }

        public Node PopTail() {
            if (count > 0) {
                Node rem = tail.prev;
                Remove(rem);
                return rem;
            }
            return null;
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache obj = new LFUCache(capacity);
 * int param_1 = obj.Get(key);
 * obj.Put(key,value);
 */
