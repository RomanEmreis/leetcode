/*
  3321. Find X-Sum of All K-Long Subarrays II
  
  You are given an array nums of n integers and two integers k and x.
  The x-sum of an array is calculated by the following procedure:
      Count the occurrences of all elements in the array.
      Keep only the occurrences of the top x most frequent elements. If two elements have the same number of occurrences, the element with the bigger value is considered more frequent.
      Calculate the sum of the resulting array.
  
  Note that if an array has less than x distinct elements, its x-sum is the sum of the array.
  Return an integer array answer of length n - k + 1 where answer[i] is the x-sum of the nums[i..i + k - 1].
  
  Example 1:
  Input: nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
  Output: [6,10,12]
  Explanation:
      For subarray [1, 1, 2, 2, 3, 4], only elements 1 and 2 will be kept in the resulting array. Hence, answer[0] = 1 + 1 + 2 + 2.
      For subarray [1, 2, 2, 3, 4, 2], only elements 2 and 4 will be kept in the resulting array. Hence, answer[1] = 2 + 2 + 2 + 4. Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.
      For subarray [2, 2, 3, 4, 2, 3], only elements 2 and 3 are kept in the resulting array. Hence, answer[2] = 2 + 2 + 2 + 3 + 3.
  
  Example 2:
  Input: nums = [3,8,7,8,7,5], k = 2, x = 2
  Output: [11,15,15,15,12]
  Explanation:
  Since k == x, answer[i] is equal to the sum of the subarray nums[i..i + k - 1].
*/
public class Solution {
    private record Entry(int Freq, int Val);

    private sealed class IndexedHeap {
        private readonly List<Entry> data = [];
        private readonly Dictionary<int, int> pos = [];
        private readonly Comparison<Entry> cmp;

        public IndexedHeap(Comparison<Entry> comparer) { 
            cmp = comparer; 
        }

        public int Count => data.Count;

        public bool Contains(int val) => pos.ContainsKey(val);

        public Entry Peek() {
            return data[0];
        }

        public Entry Pop() {
            var top = data[0];
            RemoveAt(0);
            return top;
        }

        public void InsertOrUpdate(Entry e) {
            if (pos.TryGetValue(e.Val, out int idx)) {
                var old = data[idx];
                data[idx] = e;
                if (cmp(e, old) < 0) SiftUp(idx); else SiftDown(idx);
            } else {
                pos[e.Val] = data.Count;
                data.Add(e);
                SiftUp(data.Count - 1);
            }
        }

        public void RemoveVal(int val) {
            if (!pos.TryGetValue(val, out int idx)) 
                return;
            RemoveAt(idx);
        }

        private void RemoveAt(int idx) {
            int last = data.Count - 1;
            var removed = data[idx];
            pos.Remove(removed.Val);
            if (idx == last) {
                data.RemoveAt(last);
                return;
            }

            var moved = data[last];
            data[idx] = moved;
            pos[moved.Val] = idx;

            data.RemoveAt(last);

            SiftUp(idx);
            SiftDown(idx);
        }

        private void SiftUp(int i) {
            while (i > 0) {
                int p = (i - 1) >> 1;
                if (cmp(data[i], data[p]) < 0) {
                    Swap(i, p);
                    i = p;
                } else {
                    break;
                }
            }
        }

        private void SiftDown(int i) {
            int n = data.Count;

            while (true) {
                int l = i * 2 + 1;
                if (l >= n)
                    break;
                
                int r = l + 1;
                int best = l;

                if (r < n && cmp(data[r], data[l]) < 0) 
                    best = r;

                if (cmp(data[best], data[i]) < 0) {
                    Swap(best, i);
                    i = best;
                } else {
                    break;
                }
            }
        }

        private void Swap(int i, int j) {
            var t = data[i];
            data[i] = data[j];
            data[j] = t;
            pos[data[i].Val] = i;
            pos[data[j].Val] = j;
        }
    }

    public long[] FindXSum(int[] nums, int k, int x) {
        int n = nums.Length;
        if (n < k) 
            return Array.Empty<long>();

        Dictionary<int, int> freq = [];

        var topHeap = new IndexedHeap((a, b) => {
            if (a.Freq != b.Freq) 
                return a.Freq.CompareTo(b.Freq);
            return a.Val.CompareTo(b.Val);
        });

        var restHeap = new IndexedHeap((a, b) => {
            if (a.Freq != b.Freq)
                return -a.Freq.CompareTo(b.Freq);
            return -a.Val.CompareTo(b.Val);
        });

        Dictionary<int, int> which = [];

        long sum = 0;
        long[] res = new long[n - k + 1];

        void InsertOrUpdateVal(int val, int newFreq) {
            var e = new Entry(newFreq, val);
            if (!which.TryGetValue(val, out int w) || w == 0) {
                restHeap.InsertOrUpdate(e);
                which[val] = 2;
            } else if (w == 1) {
                topHeap.InsertOrUpdate(e);
            } else {
                restHeap.InsertOrUpdate(e);
            }
        }

        void RemoveValCompletely(int val) {
            if (!which.TryGetValue(val, out int w) || w == 0) 
                return;

            if (w == 1) {
                if (freq.TryGetValue(val, out int f)) 
                    sum -= 1L * f * val;

                topHeap.RemoveVal(val);
            } else {
                restHeap.RemoveVal(val);
            }

            which[val] = 0;
        }

        void EnsureHeapRecord(int val) {
            if (!freq.TryGetValue(val, out int f)) {
                RemoveValCompletely(val); 
                return; 
            }

            if (!which.TryGetValue(val, out int w) || w == 0) {
                restHeap.InsertOrUpdate(new Entry(f, val));
                which[val] = 2;
            } else if (w == 1) {
                topHeap.InsertOrUpdate(new Entry(f, val));
            } else {
                restHeap.InsertOrUpdate(new Entry(f, val));
            }
        }

        void MoveFromRestToTop() {
            if (restHeap.Count == 0) 
                return;

            var cand = restHeap.Pop();
            
            if (!freq.TryGetValue(cand.Val, out int f) || f != cand.Freq) {
                if (freq.TryGetValue(cand.Val, out int curf))
                    restHeap.InsertOrUpdate(new Entry(curf, cand.Val));
                
                return;
            }
            
            which[cand.Val] = 1;
            topHeap.InsertOrUpdate(cand);
            sum += 1L * cand.Freq * cand.Val;
        }

        void MoveFromTopToRest() {
            if (topHeap.Count == 0)
                return;

            var worst = topHeap.Pop();
            if (!freq.TryGetValue(worst.Val, out int f) || f != worst.Freq) {
                if (freq.TryGetValue(worst.Val, out int curf)) {
                    topHeap.InsertOrUpdate(new Entry(curf, worst.Val));
                }
                return;
            }
            
            which[worst.Val] = 2;
            sum -= 1L * worst.Freq * worst.Val;
            restHeap.InsertOrUpdate(worst);
        }

        void Rebalance(int x) {
            while (true) {
                int topCount = 0;
                foreach (var kv in which) if (kv.Value == 1) 
                    ++topCount;
                break;
            }
        }

        int logicalTopCount = 0;

        void Rebalance2(int x) {
            while (logicalTopCount < x && restHeap.Count > 0) {
                var cand = restHeap.Pop();
                if (!freq.TryGetValue(cand.Val, out int curf) || curf != cand.Freq) {
                    if (freq.TryGetValue(cand.Val, out int nf))
                        restHeap.InsertOrUpdate(new Entry(nf, cand.Val));

                    continue;
                }

                if (which.TryGetValue(cand.Val, out int w) && w == 1) {
                    topHeap.InsertOrUpdate(cand);
                    continue;
                }

                which[cand.Val] = 1;
                topHeap.InsertOrUpdate(cand);
                logicalTopCount++;
                sum += 1L * cand.Freq * cand.Val;
            }

            while (restHeap.Count > 0 && topHeap.Count > 0) {
                var best = restHeap.Peek();
                if (!freq.TryGetValue(best.Val, out int bf) || bf != best.Freq) {
                    var popped = restHeap.Pop();

                    if (freq.TryGetValue(popped.Val, out int nf))
                        restHeap.InsertOrUpdate(new Entry(nf, popped.Val));

                    continue;
                }

                var worst = topHeap.Peek();
                if (!freq.TryGetValue(worst.Val, out int wf) || wf != worst.Freq) {
                    var popped = topHeap.Pop();
                    if (freq.TryGetValue(popped.Val, out int nf))
                        topHeap.InsertOrUpdate(new Entry(nf, popped.Val));

                    continue;
                }

                bool bestBetter;
                if (best.Freq != worst.Freq) 
                    bestBetter = best.Freq > worst.Freq;
                else 
                    bestBetter = best.Val > worst.Val;

                if (!bestBetter) 
                    break;

                best = restHeap.Pop();
                worst = topHeap.Pop();

                if (!freq.TryGetValue(best.Val, out int bf2) || bf2 != best.Freq) {
                    if (freq.TryGetValue(best.Val, out int nf)) 
                        restHeap.InsertOrUpdate(new Entry(nf, best.Val));
                    
                    if (freq.TryGetValue(worst.Val, out int nw)) 
                        topHeap.InsertOrUpdate(new Entry(nw, worst.Val));
                        
                    continue;
                }

                if (!freq.TryGetValue(worst.Val, out int wf2) || wf2 != worst.Freq) {
                    if (freq.TryGetValue(worst.Val, out int nw)) 
                        topHeap.InsertOrUpdate(new Entry(nw, worst.Val));

                    if (freq.TryGetValue(best.Val, out int nf2)) 
                        restHeap.InsertOrUpdate(new Entry(nf2, best.Val));

                    continue;
                }

                
                which[worst.Val] = 2;
                which[best.Val] = 1;
                sum -= 1L * worst.Freq * worst.Val;
                sum += 1L * best.Freq * best.Val;
                
                topHeap.InsertOrUpdate(best);
                restHeap.InsertOrUpdate(worst);
            }

            while (logicalTopCount > x) {
                var worst = topHeap.Pop();
                if (!freq.TryGetValue(worst.Val, out int wf2) || wf2 != worst.Freq) {
                    if (freq.TryGetValue(worst.Val, out int nf)) 
                        topHeap.InsertOrUpdate(new Entry(nf, worst.Val));
                    continue;
                }
                
                which[worst.Val] = 2;
                sum -= 1L * worst.Freq * worst.Val;
                restHeap.InsertOrUpdate(worst);
                logicalTopCount--;
            }
        }

        for (int i = 0; i < n; ++i) {
            int val = nums[i];
            freq.TryGetValue(val, out int of);
            int nf = of + 1;
            freq[val] = nf;

            if (!which.TryGetValue(val, out int w) || w == 0) {
                restHeap.InsertOrUpdate(new Entry(nf, val));
                which[val] = 2;
            }
            else if (w == 1) {
                topHeap.InsertOrUpdate(new Entry(nf, val));
                sum += 1L * val * (nf - of);
            } else {
                restHeap.InsertOrUpdate(new Entry(nf, val));
            }

            Rebalance2(x);

            int start = i - k + 1;
            if (start >= 0) {
                res[start] = sum;

                int outv = nums[start];
                freq.TryGetValue(outv, out int ofreq);
                int nfreq = ofreq - 1;
                if (nfreq == 0) {
                    freq.Remove(outv);
                } else {
                    freq[outv] = nfreq;
                }

                which.TryGetValue(outv, out int wh);
                if (wh == 1) {
                    sum -= 1L * outv;
                    
                    if (nfreq == 0) {
                        topHeap.RemoveVal(outv); 
                        which[outv] = 0; 
                        --logicalTopCount;
                    } else {
                        topHeap.InsertOrUpdate(new Entry(nfreq, outv));
                    }
                } else if (wh == 2) {
                    if (nfreq == 0) { 
                        restHeap.RemoveVal(outv);
                        which[outv] = 0;
                    } else {
                        restHeap.InsertOrUpdate(new Entry(nfreq, outv));
                    }
                }
                Rebalance2(x);
            }
        }

        return res;
    }
}
