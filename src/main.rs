use crate::todos::*;
use clap::Parser;
use sled::Db;

mod todos;

#[derive(clap::Subcommand)]
enum Action {
    #[clap(about = "Remove a task")]
    Remove { key: u64 },

    #[clap(about = "View all tasks")]
    View,

    #[clap(about = "Add a new task")]
    Add { task: String },
}

#[derive(Parser)]
struct Arguments {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let db: Db = sled::open("todos_db").expect("could not get database");
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Remove { key } => {
            match remove_todo(&db, key) {
                Ok(_) => println!("removed task {}", key),
                Err(e) => println!("{}", e),
            }
            print_todos(&db);
        }
        Action::View => {
            print_todos(&db);
        }
        Action::Add { task } => {
            add_todo(&db, task);
            print_todos(&db);
        }
    }
}
