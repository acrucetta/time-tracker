use clap::{arg, command, Command};

mod config;
mod task;

fn main() {
    // Load the config file
    let config = config::load_config();

    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("start")
                .about("Start timing a task")
                .arg(arg!([TASK]))
                .arg(arg!(--tags[TAGS])),
        )
        .subcommand(Command::new("stop").about("Stop timing a task"))
        .subcommand(Command::new("status").about("Show the current status"))
        .subcommand(Command::new("ls").about("List all tasks"))
        .get_matches();

    let subcommand = matches.subcommand();
    let (subcommand, sub_m) = if let Some(subc) = subcommand {
        subc
    } else {
        eprintln!("Missing subcommand.");
        return;
    };

    match subcommand {
        "start" => {
            todo!();
        }
        "stop" => {
            todo!();
        }
        "status" => {
            todo!();
        }
        "ls" => {
            todo!();
        }
        otherwise => {
            eprintln!("Unknown subcommand: {}", otherwise);
        }
    }
}
