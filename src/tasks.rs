use chrono::{Date, DateTime, Duration, Local, NaiveDate, TimeZone, Utc};
use colored::Colorize;
use enum_iterator::{all, Sequence};
use num_derive::FromPrimitive;
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
    pub comments: Option<String>,
}

#[derive(Debug, FromPrimitive, Sequence)]
pub enum PredefinedTasks {
    Meetings = 1,
    Reading = 2,
    Journaling = 3,
    Hobby_Code = 4,
    Work_Code = 5,
    BrowseInternet = 6,
    Anki = 7,
    Other = 8,
}

// Add String::from to the predefined tasks
impl std::fmt::Display for PredefinedTasks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PredefinedTasks::Meetings => write!(f, "{}", "Meetings"),
            PredefinedTasks::Reading => write!(f, "{}", "Reading"),
            PredefinedTasks::Journaling => write!(f, "{}", "Journaling"),
            PredefinedTasks::BrowseInternet => write!(f, "{}", "Browse Internet"),
            PredefinedTasks::Other => write!(f, "{}", "Other"),
            PredefinedTasks::Hobby_Code => write!(f, "{}", "Hobby Code"),
            PredefinedTasks::Work_Code => write!(f, "{}", "Work Code"),
            PredefinedTasks::Anki => write!(f, "{}", "Anki"),
        }
    }
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
        writeln!(
            f,
            "{}: {}",
            "Comments:".bright_yellow(),
            self.comments.as_ref().unwrap_or(&String::from("None"))
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
            comments: None,
        }
    }

    pub fn end_task(&mut self, energy: i8, comment: String) {
        let now = chrono::Local::now();
        self.end_time = Some(now);
        self.duration = now - self.start_time;

        if self.duration.num_minutes() > 120 {
            print!("The task took a long time; can you confirm the actual duration? (N min): ");
            let mut confirm = String::new();
            io::stdin()
                .read_line(&mut confirm)
                .expect("Failed to read line");
            let confirm = confirm.trim().parse::<i64>().unwrap();
            self.duration = chrono::Duration::minutes(confirm);
        }
        self.energy = Some(energy);
        self.comments = Some(comment);
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
        let comments = match record[7].is_empty() {
            true => None,
            false => Some(record[7].to_string()),
        };
        Task {
            id,
            name,
            start_time,
            end_time,
            duration,
            tags,
            energy,
            comments,
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

    // This function prompts the user to select
    // a task from a list of predefined tasks
    pub fn get_task_name(&self) -> String {
        let mut input = String::new();
        println!("Select a task:");

        // Print the list of predefined tasks from the enum
        // using the Sequence trait
        let predefined_tasks = all::<PredefinedTasks>().collect::<Vec<_>>();
        for (i, task) in predefined_tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
        io::stdin().read_line(&mut input).unwrap();
        let task_number = input.trim().parse::<usize>().unwrap();
        let task_name = predefined_tasks[task_number - 1].to_string();
        task_name
    }

    pub fn create_task(&mut self) -> TimeTrackerResult {
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

        task.name = self.get_task_name();
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

    fn get_comment_from_user() -> String {
        let mut comment = String::new();
        println!("How did the task go?: ");
        io::stdin()
            .read_line(&mut comment)
            .expect("Failed to write comment");
        comment
    }

    pub fn stop_active_task(&mut self) -> TimeTrackerResult {
        // We want to get the last task that has no end time
        // and stop it
        let last_task = self.tasks.last_mut().unwrap();
        if last_task.end_time.is_none() {
            let energy = TimeTracker::get_energy_from_user();
            let comment = TimeTracker::get_comment_from_user();
            last_task.end_task(energy, comment);
            return TimeTrackerResult::Success;
        }

        TimeTrackerResult::Error(TimeTrackerError::NoActiveTasks)
    }

    pub fn add_manual_task(&mut self) -> TimeTrackerResult {
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
        let task_name = self.get_task_name();

        // Ask the user for the date
        let mut input = String::new();
        println!("Enter task date (YYYY-MM-DD): ");
        io::stdin().read_line(&mut input).unwrap();

        // Parse to DateTime Chrono object
        let task_date = NaiveDate::parse_from_str(&input.trim(), "%Y-%m-%d").unwrap();
        let task_date_time = task_date.and_hms_opt(0, 0, 0).unwrap();
        let task_start_time: chrono::DateTime<chrono::Local> =
            Local.from_local_datetime(&task_date_time).unwrap();

        // Get task duration
        let mut input = String::new();
        println!("Enter task duration in minutes (0-120): ");
        io::stdin().read_line(&mut input).unwrap();
        let task_duration = input.trim().parse::<i64>().unwrap();

        // Get task energy
        let task_energy = TimeTracker::get_energy_from_user();

        // Get task comment
        let task_comment = TimeTracker::get_comment_from_user();

        task.name = task_name;
        task.id = id;
        task.duration = Duration::minutes(task_duration);
        task.start_time = task_start_time;
        task.end_time = Some(task_start_time + Duration::minutes(task_duration));
        task.energy = Some(task_energy);
        task.comments = Some(task_comment);
        self.tasks.push(task);
        TimeTrackerResult::Success
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
            "comments",
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
                match task.comments {
                    Some(c) => c,
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
