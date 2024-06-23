/*
  Given an array of integers nums and an integer limit, 
  return the size of the longest non-empty subarray such that the absolute difference between any two elements of this subarray is less than or equal to limit.

  Example:
    Input: nums = [8,2,4,7], limit = 4
    Output: 2 
    Explanation: All subarrays are: 
    [8] with maximum absolute diff |8-8| = 0 <= 4.
    [8,2] with maximum absolute diff |8-2| = 6 > 4. 
    [8,2,4] with maximum absolute diff |8-2| = 6 > 4.
    [8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
    [2] with maximum absolute diff |2-2| = 0 <= 4.
    [2,4] with maximum absolute diff |2-4| = 2 <= 4.
    [2,4,7] with maximum absolute diff |2-7| = 5 > 4.
    [4] with maximum absolute diff |4-4| = 0 <= 4.
    [4,7] with maximum absolute diff |4-7| = 3 <= 4.
    [7] with maximum absolute diff |7-7| = 0 <= 4. 
  Therefore, the size of the longest subarray is 2.
*/
#define MAX_SIZE 10000

struct Deque {
    int arr[MAX_SIZE];
    int front, rear, size;
};

void push_front(struct Deque* deque, int value) {
    deque->front = (deque->front-1+MAX_SIZE) % MAX_SIZE;
    deque->arr[deque->front] = value;
    deque->size++;
}

void push_back(struct Deque* deque, int value) {
    deque->arr[deque->rear] = value;
    deque->rear = (deque->rear+1) % MAX_SIZE;
    deque->size++;
}

void pop_front(struct Deque* deque) {
    deque->front = (deque->front+1) % MAX_SIZE;
    deque->size--;
}

void pop_back(struct Deque* deque) {
    deque->rear = (deque->rear-1+MAX_SIZE) % MAX_SIZE;
    deque->size--;
}

int front(struct Deque* deque) {
    return deque->arr[deque->front];
}

int back(struct Deque* deque) {
    return deque->arr[(deque->rear-1+MAX_SIZE) % MAX_SIZE];
}

int size(struct Deque* deque) {
    return deque->size;
}

int longestSubarray(int* nums, int numsSize, int limit) {
    if (numsSize == 1) return 1;

    struct Deque* minDeque = malloc(sizeof(struct Deque));
    struct Deque* maxDeque = malloc(sizeof(struct Deque));

    minDeque->front = 0; minDeque->rear = 0; minDeque->size = 0;
    maxDeque->front = 0; maxDeque->rear = 0; maxDeque->size = 0;

    int i = 0, result = 0;
    for (int j = 0; j < numsSize; ++j) {
        while (size(minDeque) != 0 && nums[j] < nums[back(minDeque)]) pop_back(minDeque);
        while (size(maxDeque) != 0 && nums[j] > nums[back(maxDeque)]) pop_back(maxDeque);

        push_back(minDeque, j);
        push_back(maxDeque, j);

        while (nums[front(maxDeque)] - nums[front(minDeque)] > limit) {
            if (front(minDeque) == i) pop_front(minDeque);
            if (front(maxDeque) == i) pop_front(maxDeque);
            ++i;
        }

        result = result > (j - i + 1) ? result : (j - i + 1);
    }

    free(minDeque);
    free(maxDeque);
    
    return result;
}
