/*
  3408. Design Task Manager
  
  There is a task management system that allows users to manage their tasks, each associated with a priority. The system should efficiently handle adding, modifying, executing, and removing tasks.
  Implement the TaskManager class:
  TaskManager(vector<vector<int>>& tasks) initializes the task manager with a list of user-task-priority triples. Each element in the input list is of the form [userId, taskId, priority], which adds a task to the specified user with the given priority.
  void add(int userId, int taskId, int priority) adds a task with the specified taskId and priority to the user with userId. It is guaranteed that taskId does not exist in the system.
  void edit(int taskId, int newPriority) updates the priority of the existing taskId to newPriority. It is guaranteed that taskId exists in the system.
  void rmv(int taskId) removes the task identified by taskId from the system. It is guaranteed that taskId exists in the system.
  int execTop() executes the task with the highest priority across all users. If there are multiple tasks with the same highest priority, execute the one with the highest taskId.
  After executing, the taskId is removed from the system. Return the userId associated with the executed task. If no tasks are available, return -1.
  
  Note that a user may be assigned multiple tasks.
  
  Example 1:
  Input:
  ["TaskManager", "add", "edit", "execTop", "rmv", "add", "execTop"]
  [[[[1, 101, 10], [2, 102, 20], [3, 103, 15]]], [4, 104, 5], [102, 8], [], [101], [5, 105, 15], []]
  
  Output:
  [null, null, null, 3, null, null, 5]
  
  Explanation
  TaskManager taskManager = new TaskManager([[1, 101, 10], [2, 102, 20], [3, 103, 15]]); // Initializes with three tasks for Users 1, 2, and 3.
  taskManager.add(4, 104, 5); // Adds task 104 with priority 5 for User 4.
  taskManager.edit(102, 8); // Updates priority of task 102 to 8.
  taskManager.execTop(); // return 3. Executes task 103 for User 3.
  taskManager.rmv(101); // Removes task 101 from the system.
  taskManager.add(5, 105, 15); // Adds task 105 with priority 15 for User 5.
  taskManager.execTop(); // return 5. Executes task 105 for User 5.
*/
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Task {
    priority: i32,
    task_id: i32,
}

impl Ord for Task {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| self.task_id.cmp(&other.task_id))
    }
}

impl PartialOrd for Task {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskManager {
    task_info: HashMap<i32, (i32, i32)>,
    heap: BinaryHeap<Task>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let n = tasks.len() * 2;
        let mut task_info = HashMap::with_capacity(n);
        let mut heap = BinaryHeap::with_capacity(n);
        for task in tasks.iter() {
            let user_id = task[0];
            let task_id = task[1];
            let priority = task[2];
            
            task_info.insert(task_id, (priority, user_id));
            heap.push(Task { priority, task_id });
        }
        Self { task_info, heap }
    }
    
    #[inline]
    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_info.insert(task_id, (priority, user_id));
        self.heap.push(Task { priority, task_id });
    }
    
    #[inline]
    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(info) = self.task_info.get_mut(&task_id) {
            info.0 = new_priority;
            self.heap.push(Task { priority: new_priority, task_id });
        }
    }
    
    #[inline]
    fn rmv(&mut self, task_id: i32) {
        self.task_info.remove(&task_id);
    }
    
    #[inline]
    fn exec_top(&mut self) -> i32 {
        while let Some(task) = self.heap.pop() {
            if let Some(&(priority, user_id)) = self.task_info.get(&task.task_id) {
                if priority == task.priority {
                    self.task_info.remove(&task.task_id);
                    return user_id;
                }
            }
        }
        -1
    }
}
