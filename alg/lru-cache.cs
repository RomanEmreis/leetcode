/*
  Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
  
  Implement the LRUCache class:
  
  LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
  int get(int key) Return the value of the key if the key exists, otherwise return -1.
  void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
  The functions get and put must each run in O(1) average time complexity.
*/
using System.Runtime.CompilerServices;

public sealed class Node {
    public int key, value;
    public Node next, prev;

    public Node(int key = 0, int value = 0) {
        this.key = key;
        this.value = value;
    }
}

public sealed class LRUCache {
    private readonly Dictionary<int, Node> cache;
    private readonly int capacity;

    private Node head, tail;

    public LRUCache(int capacity) {
        cache = new(capacity);
        this.capacity = capacity;

        head = new();
        tail = new();

        head.next = tail;
        tail.prev = head;
    }
    
    public int Get(int key) {
        if (cache.ContainsKey(key)) {
            Node node = cache[key];
            MoveToHead(node);
            return node.value;
        }
        return -1;
    }
    
    public void Put(int key, int value) {
        if (cache.ContainsKey(key)) {
            Node node = cache[key];
            node.value = value;
            MoveToHead(node);
        } else {
            Node node = new(key, value);
            cache.Add(key, node);
            Add(node);

            if (cache.Count > capacity) {
                Node lru = PopTail();
                cache.Remove(lru.key);
            }
        }
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private void MoveToHead(Node node) {
        Remove(node);
        Add(node);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private void Remove(Node node) {
        Node next = node.next;
        Node prev = node.prev;

        prev.next = next;
        next.prev = prev;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private void Add(Node node) {
        node.prev = head;
        node.next = head.next;

        head.next.prev = node;
        head.next = node;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private Node PopTail() {
        Node rem = tail.prev;
        Remove(rem);
        return rem;
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache obj = new LRUCache(capacity);
 * int param_1 = obj.Get(key);
 * obj.Put(key,value);
 */
