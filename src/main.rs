use clap::{arg, command, Command};
use tasks::{TimeTracker, TimeTrackerResult};

mod config;
mod tasks;

fn main() {
    // Load the config file
    let config = config::load_config();

    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Start timing a task")
                .arg(arg!(--type[TYPE])),
        )
        .subcommand(Command::new("stop").about("Stop timing a task"))
        .subcommand(Command::new("status").about("Show the current status"))
        .subcommand(Command::new("ls").about("List all tasks"))
        .get_matches();

    let mut time_tracker = match TimeTracker::from_file(&config.tasks_path) {
        Ok(time_tracker) => time_tracker,
        Err(_) => TimeTracker::new(),
    };

    let subcommand = matches.subcommand();
    let (subcommand, sub_m) = if let Some(subc) = subcommand {
        subc
    } else {
        eprintln!("Missing subcommand.");
        return;
    };

    match subcommand {
        "add" => {
            if let Some(tag) = sub_m.get_one::<String>("type") {
                time_tracker.create_manual_task();
            } else {
                time_tracker.create_task();
            }
        }
        "stop" => {
            time_tracker.stop_active_task();
        }
        "status" => {
            time_tracker.show_active_tasks();
        }
        "ls" => {
            time_tracker.show_all_tasks();
        }
        otherwise => {
            eprintln!("Unknown subcommand: {}", otherwise);
        }
    }

    // Save the tasks to file
    TimeTracker::save_to_file(time_tracker, &config.tasks_path).unwrap();
}
