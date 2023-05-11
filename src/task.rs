use chrono;

pub struct Task {
    pub id: u32,
    pub name: String,
    pub start_time: chrono::DateTime<chrono::Local>,
    pub end_time: Option<chrono::DateTime<chrono::Local>>,
    pub duration: chrono::Duration,
    pub tags: Option<Vec<String>>,
    pub energy: Option<i8>, // How much energy did this task give me?
}

impl Task {
    pub fn new() {
        let id: u32 = 0;
        let name: String = String::from("");
        let start_time: chrono::DateTime<chrono::Local> = chrono::Local::now();
        let end_time: Option<chrono::DateTime<chrono::Local>> = None;
        let duration: chrono::Duration = chrono::Duration::seconds(0);
        let tags: Vec<String> = Vec::new();
        let energy: i8 = 0;
    }

    pub fn end_task(&mut self, energy: i8) {
        let now = chrono::Local::now();
        self.end_time = Some(now);
        self.duration = now - self.start_time;
        self.energy = Some(energy);
    }
}

pub struct TimeTracker {
    pub tasks: Vec<Task>,
}

pub enum TimeTrackerError {
    InvalidTaskName,
    InvalidTaskDuration,
    InvalidTaskTags,
    InvalidTaskEnergy,
}

pub enum TimeTrackerResult {
    Success,
    Error(TimeTrackerError),
}

impl TimeTracker {
    pub fn new() {
        let tasks: Vec<Task> = Vec::new();
    }

    pub fn create_task() {
        todo!("Create a task")
    }
}
