use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String
    },
    /// Remove an entry from the journal file by position
    Done {
        #[structopt()]
        position: usize
    },
    /// List incompleted tasks in the journal file
    List,
    /// Modify an existing entry using its position
    Edit {
        /// The position of the task to edit.
        #[structopt()]
        position: usize,
        /// The new text to replace the old text.
        #[structopt()]
        text: String
    },
    /// List completed tasks in the journal file
    ListCompleted,
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "Rusty Journal",
about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}