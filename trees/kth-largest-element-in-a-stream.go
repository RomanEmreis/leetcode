/*
  Design a class to find the kth largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.
  
  Implement KthLargest class:
    KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
    int add(int val) Appends the integer val to the stream and returns the element representing the kth largest element in the stream.

  Example 1:
    Input
    ["KthLargest", "add", "add", "add", "add", "add"]
    [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
    Output
    [null, 4, 5, 5, 8, 8]
    Explanation
    KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
    kthLargest.add(3);   // return 4
    kthLargest.add(5);   // return 5
    kthLargest.add(10);  // return 5
    kthLargest.add(9);   // return 8
    kthLargest.add(4);   // return 8
*/
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) };
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] };
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] };

func (h *IntHeap) Push(x interface{}) {
	*h = append(*h, x.(int));
}

func (h *IntHeap) Pop() interface{} {
	old := *h;
	n := len(old);
	x := old[n - 1];
	*h = old[0 : n - 1];
	return x;
}

type KthLargest struct {
    k    int;
	heap *IntHeap;
}


func Constructor(k int, nums []int) KthLargest {
    h := &IntHeap{};
	heap.Init(h);
	kth := KthLargest{k, h};
	for _, num := range nums {
		kth.Add(num);
	}
	return kth;
}

func (this *KthLargest) Add(val int) int {
    heap.Push(this.heap, val);
    if this.heap.Len() > this.k {
        heap.Pop(this.heap);
    }
	return (*this.heap)[0];
}


/**
 * Your KthLargest object will be instantiated and called as such:
 * obj := Constructor(k, nums);
 * param_1 := obj.Add(val);
 */
