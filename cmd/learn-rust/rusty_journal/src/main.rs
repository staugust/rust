mod cli;
mod tasks;
use structopt::StructOpt;
use cli::{CommandLineArgs, Action::*};
use std::path::PathBuf;
use anyhow::anyhow;
fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()>{
    let CommandLineArgs{
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();
    let journal_file = journal_file.or_else(find_default_journal_file)
        .ok_or(anyhow!("failed to found journal file."))?;

    match action{
        Add {text} => {
            tasks::add_task(journal_file, tasks::Task::new(text))
        },
        List => {
            tasks::list_tasks(journal_file)
        },
        Done{position} => {
            tasks::complete_task(journal_file, position)
        },
    }?;
    Ok(())
}
