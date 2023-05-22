use chrono::{Local, TimeZone};
use colored::Colorize;
use std::io;
use std::{self, fmt};

const SAVE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f %z";
const SHOW_TIME_FORMAT: &str = "%Y-%m-%d %H:%M";

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
        let tags = match &self.tags {
            Some(tags) => tags.join(","),
            None => String::from("None"),
        };
        writeln!(f, "{}{}", "@".bright_black(), tags.bright_black())?;
        writeln!(f, "{} {}", "Task:".bright_yellow(), self.name.black())?;
        writeln!(
            f,
            "{}{}",
            "Start: ".bright_yellow(),
            self.start_time.format(SHOW_TIME_FORMAT)
        )?;
        writeln!(
            f,
            "{}{}",
            "End: ".bright_yellow(),
            match self.end_time {
                Some(end_time) => end_time.format(SHOW_TIME_FORMAT).to_string(),
                None => String::from("In Progress..."),
            }
        )?;
        writeln!(
            f,
            "{}: {} minutes",
            "Duration".bright_yellow(),
            // If the task is in progress, show the duration as the time since the start
            // Otherwise, show the duration as the time between start and end
            match self.end_time {
                Some(end_time) => (end_time - self.start_time).num_minutes(),
                None => (chrono::Local::now() - self.start_time).num_minutes(),
            }
        )?;
        writeln!(
            f,
            "{}: {}",
            "Life Energy".bright_yellow(),
            self.energy.unwrap_or(0)
        )?;
        Ok(())
    }
}

impl Task {
    pub fn new() -> Task {
        let curr_time = chrono::Local::now();

        Task {
            id: 0,
            name: String::from(""),
            start_time: curr_time,
            end_time: None,
            duration: chrono::Duration::seconds(0),
            tags: None,
            energy: None,
        }
    }

    pub fn end_task(&mut self, energy: i8) {
        let now = chrono::Local::now();
        self.end_time = Some(now);
        self.duration = now - self.start_time;
        self.energy = Some(energy);
    }

    fn from_record(record: csv::StringRecord) -> Task {
        // Load a task from a CSV record
        let id = record[0].parse::<u32>().unwrap();
        let name = record[1].to_string();
        let start_time = Local
            .datetime_from_str(&record[2], SAVE_TIME_FORMAT)
            .unwrap();
        let end_time = match record[3].is_empty() {
            true => None,
            false => Some(
                Local
                    .datetime_from_str(&record[3], SAVE_TIME_FORMAT)
                    .unwrap(),
            ),
        };
        let duration = if record[4].is_empty() {
            chrono::Duration::seconds(0)
        } else {
            chrono::Duration::seconds(record[4].parse::<i64>().unwrap())
        };
        let tags = match record[5].is_empty() {
            true => None,
            false => Some(record[5].split(',').map(|s| s.to_string()).collect()),
        };
        let energy = match record[6].is_empty() {
            true => None,
            false => Some(record[6].parse::<i8>().unwrap()),
        };
        Task {
            id,
            name,
            start_time,
            end_time,
            duration,
            tags,
            energy,
        }
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
    TaskAlreadyActive,
}

impl fmt::Display for TimeTrackerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeTrackerError::InvalidTaskName => write!(f, "Invalid task name"),
            TimeTrackerError::InvalidTaskDuration => write!(f, "Invalid task duration"),
            TimeTrackerError::InvalidTaskTags => write!(f, "Invalid task tags"),
            TimeTrackerError::InvalidTaskEnergy => write!(f, "Invalid task energy"),
            TimeTrackerError::InvalidTaskId => write!(f, "Invalid task id"),
            TimeTrackerError::NoActiveTasks => write!(f, "No active tasks"),
            TimeTrackerError::TaskAlreadyActive => write!(f, "Task already active"),
        }
    }
}

pub enum TimeTrackerResult {
    Success,
    Error(TimeTrackerError),
}

impl TimeTracker {
    pub fn new() -> TimeTracker {
        let tasks: Vec<Task> = Vec::new();
        TimeTracker { tasks }
    }

    fn check_if_task_is_active(&self) -> bool {
        // Check if there is an active task
        for task in self.tasks.iter() {
            if task.end_time.is_none() {
                return true;
            }
        }
        false
    }

    pub fn create_task(&mut self, name: &str, tags: &str) -> TimeTrackerResult {
        // Check if there is an active task
        if self.check_if_task_is_active() {
            return TimeTrackerResult::Error(TimeTrackerError::TaskAlreadyActive);
        }

        // Check the last task id
        let id = match self.tasks.last() {
            Some(task) => task.id + 1,
            None => 1,
        };

        let mut task = Task::new();
        task.name = name.to_string();
        task.tags = Some(tags.split(',').map(|s| s.to_string()).collect());
        task.id = id;
        self.tasks.push(task);
        TimeTrackerResult::Success
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

    pub fn show_all_tasks(&self) -> TimeTrackerResult {
        // Show all tasks
        for task in self.tasks.iter() {
            println!("{}", task);
        }
        TimeTrackerResult::Success
    }

    fn get_energy_from_user() -> i8 {
        // Get energy from user
        let mut energy = String::new();
        println!("How much life energy did this task give you? (1-10): ");
        io::stdin()
            .read_line(&mut energy)
            .expect("Failed to read line");
        let energy: i8 = energy.trim().parse().expect("Please type a number!");
        energy
    }

    pub fn stop_active_task(&mut self) -> TimeTrackerResult {
        // We want to get the last task that has no end time
        for task in self.tasks.iter_mut().rev() {
            if task.end_time.is_none() {
                let energy = TimeTracker::get_energy_from_user();
                task.end_task(energy);
                return TimeTrackerResult::Success;
            }
        }
        TimeTrackerResult::Error(TimeTrackerError::NoActiveTasks)
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

    /// Save all tasks to a csv file
    pub fn save_to_file(
        time_tracker: TimeTracker,
        file_path: &str,
    ) -> Result<TimeTrackerResult, csv::Error> {
        // Save tasks to csv
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_path(file_path)?;

        writer.write_record([
            "id",
            "name",
            "start_time",
            "end_time",
            "duration",
            "tags",
            "energy",
        ])?;

        for task in time_tracker.tasks {
            writer.write_record([
                task.id.to_string(),
                task.name.clone(),
                task.start_time.to_string(),
                match task.end_time {
                    Some(t) => t.to_string(),
                    None => String::from(""),
                },
                task.duration.num_seconds().to_string(),
                match task.tags {
                    Some(t) => t.join(","),
                    None => String::from(""),
                },
                match task.energy {
                    Some(e) => e.to_string(),
                    None => String::from(""),
                },
            ])?;
        }
        writer.flush()?;
        Ok(TimeTrackerResult::Success)
    }

    pub fn from_file(file_path: &str) -> Result<TimeTracker, csv::Error> {
        // Read a CSV file and return a TaskManager
        let mut tasks: Vec<Task> = Vec::new();

        // Read the CSV file
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(file_path);

        let mut rdr = match rdr {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(TimeTracker::new());
            }
        };

        for result in rdr.records() {
            let record = result?;
            if &record[0] == "id" {
                continue;
            }
            if record.is_empty() {
                continue;
            }
            let task = Task::from_record(record);
            tasks.push(task);
        }
        Ok(TimeTracker { tasks })
    }
}
