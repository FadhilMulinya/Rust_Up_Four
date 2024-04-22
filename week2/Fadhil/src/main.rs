//creating a struct with the following items
struct  Task {
    id: i32,  //representing the task id
    description: String,     //representing the task description
    completed: bool,    //this indicates whether a task is finished or not
}

//struct to mange tasks
struct TaskManager {
    tasks: Vec<Task>, //for managing the tasks vector
    counter: i32, //for generating unique ids
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            counter: 1,
        }
    }

    //function to add a new task
    fn add_task (&mut, description: &str) -> Task{
        let new_task = Task {
            id: self.counter.
            description: description.to_string(),
            completed: false,
        };
        //increment the counter for the next task
        self.counter +=1;
        //add the task to the vector
        self.task.push(new_task);
        //Return the created task
        new_task
    }
}

fn main(){
    //create a neew task manager instance
    let mut task_manager = TaskManager::new();

    //add a a task
    let new_task = task_manager.add_task("Complete assignment");
    println!("Added task: {?}", new_task);
}
