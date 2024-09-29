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
struct Node {
    int count;
    unordered_set<string> keys;
};

class AllOne {
public:
    AllOne() {
        ios_base::sync_with_stdio(0);
        cin.tie(0); cout.tie(0);
    }
    
    void inc(string key) {
        if (const auto it = keyToIterator.find(key); it == keyToIterator.end())
            addNewKey(key);
        else
            incrementExistingKey(it, key);
    }

    void dec(string key) {
        const auto it = keyToIterator.find(key);
        decrementExistingKey(it, key);
    }

    string getMaxKey() {
        return nodeList.empty() ? "" : *nodeList.back().keys.begin();
    }

    string getMinKey() {
        return nodeList.empty() ? "" : *nodeList.front().keys.begin();
    }

private:
    list<Node> nodeList;
    unordered_map<string, list<Node>::iterator> keyToIterator;

    void addNewKey(const string& key) {
        if (nodeList.empty() || nodeList.front().count > 1) nodeList.push_front({1, {key}});
        else nodeList.front().keys.insert(key);
        keyToIterator[key] = nodeList.begin();
    }

    void incrementExistingKey(unordered_map<string, list<Node>::iterator>::iterator it, const string& key) {
        const auto listIt = it->second;

        auto nextIt = next(listIt);
        const int newCount = listIt->count + 1;
        if (nextIt == nodeList.end() || nextIt->count > newCount) nextIt = nodeList.insert(nextIt, {newCount, {key}});
        else nextIt->keys.insert(key);

        keyToIterator[key] = nextIt;
        remove(listIt, key);
    }

    void decrementExistingKey(unordered_map<string, list<Node>::iterator>::iterator it,const string& key) {
        const auto listIt = it->second;
        if (listIt->count == 1) {
            keyToIterator.erase(it);
            remove(listIt, key);
            return;
        }

        auto prevIt = prev(listIt);
        const int newCount = listIt->count - 1;
        if (listIt == nodeList.begin() || prevIt->count < newCount) prevIt = nodeList.insert(listIt, {newCount, {key}});
        else prevIt->keys.insert(key);

        keyToIterator[key] = prevIt;
        remove(listIt, key);
    }

    void remove(list<Node>::iterator it, const string& key) {
        it->keys.erase(key);
        if (it->keys.empty()) nodeList.erase(it);
    }
};

/**
 * Your AllOne object will be instantiated and called as such:
 * AllOne* obj = new AllOne();
 * obj->inc(key);
 * obj->dec(key);
 * string param_3 = obj->getMaxKey();
 * string param_4 = obj->getMinKey();
 */
