use tokio::task::JoinHandle;
pub struct TasksPool {
    pub pool: Vec<Task>,
}

impl TasksPool {
    pub fn new() -> Self {
        Self { pool: Vec::new() }
    }
    pub fn add(&mut self, task: Task) {
        //
        self.pool.push(task);
    }
    pub async fn join(self) {
        for task in self.pool {
            task.handler.await.unwrap();
        }
    }
}

#[allow(dead_code)]
pub struct Task {
    pub id: u32,
    pub handler: JoinHandle<()>,
}

impl Task {
    pub fn new(id: u32, handler: JoinHandle<()>) -> Self {
        //
        Self { id, handler }
    }
}
