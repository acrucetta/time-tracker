use chrono;
use std::{self, fmt};

pub struct Task {
    pub id: u32,
    pub name: String,
    pub start_time: chrono::DateTime<chrono::Local>,
    pub end_time: Option<chrono::DateTime<chrono::Local>>,
    pub duration: chrono::Duration,
    pub tags: Option<Vec<String>>,
    pub energy: Option<i8>, // How much energy did this task give me?
}

impl fmt::Display for Task {
    /// Formats the value using the given formatter.
    ///
    /// E.g.,
    /// Task: Task description
    /// Start: 2020-01-01 00:00:00
    /// End: 2020-01-01 00:00:00
    /// Duration: 00:00:00
    /// Energy: 0
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task: {}\n", self.name)?;
        write!(f, "Start: {}\n", self.start_time)?;
        write!(
            f,
            "End: {}\n",
            self.end_time.unwrap_or(chrono::Local::now())
        )?;
        write!(f, "Duration: {}\n", self.duration)?;
        write!(f, "Energy: {}\n", self.energy.unwrap_or(0))
    }
}

impl Task {
    pub fn new() -> Task {
        let task = Task {
            id: 0,
            name: String::from(""),
            start_time: chrono::Local::now(),
            end_time: None,
            duration: chrono::Duration::seconds(0),
            tags: None,
            energy: None,
        };
        task
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
    InvalidTaskId,
    NoActiveTasks,
}

pub enum TimeTrackerResult {
    Success,
    Error(TimeTrackerError),
}

impl TimeTracker {
    pub fn new() {
        let tasks: Vec<Task> = Vec::new();
    }

    pub fn create_task(&mut self) -> TimeTrackerResult {
        let task = Task::new();
        self.tasks.push(task);
        TimeTrackerResult::Success
    }

    pub fn remove_task(&mut self, id: u32) -> TimeTrackerResult {
        // Remove task with id
        for (i, task) in self.tasks.iter().enumerate() {
            if task.id == id {
                self.tasks.remove(i);
                return TimeTrackerResult::Success;
            }
        }
        TimeTrackerResult::Error(TimeTrackerError::InvalidTaskId)
    }

    pub fn show_active_tasks(&self) -> TimeTrackerResult {
        // Show active task
        for task in self.tasks.iter() {
            if task.end_time.is_none() {
                println!("{}", task);
                return TimeTrackerResult::Success;
            }
        }
        TimeTrackerResult::Error(TimeTrackerError::NoActiveTasks)
    }

    /// Save all tasks to a csv file
    pub fn save_tasks_to_csv(&self) -> TimeTrackerResult {
        // Save tasks to csv
        TimeTrackerResult::Success
    }

    pub fn load_tasks_from_csv(&mut self) -> TimeTrackerResult {
        // Load tasks from csv
        TimeTrackerResult::Success
    }
}
