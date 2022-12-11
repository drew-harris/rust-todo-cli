use clap::Parser;
use sled::Db;

#[derive(clap::Subcommand)]
enum Action {
    Remove,
    Complete,
    View,

    #[clap(about = "Add a new task")]
    Add {
        task: String,
    },
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
        Action::Remove => println!("remove"),
        Action::Complete => println!("complete"),
        Action::View => {
            db.iter().for_each(|todo_item| match todo_item {
                Ok((key, value)) => {
                    let key = String::from_utf8(key.to_vec()).unwrap();
                    let value = String::from_utf8(value.to_vec()).unwrap();
                    println!("{}: {}", key, value);
                }
                Err(e) => println!("error: {}", e),
            });
        }

        Action::Add { task } => {
            let key = db
                .iter()
                .map(|item| {
                    let key = item.expect("could not get key").0;
                    let key = String::from_utf8(key.to_vec()).expect("could not convert to string");
                    let key = key.parse::<u64>().expect("could not parse key");
                    key + 1
                })
                .last()
                .unwrap_or(0);

            // Insert task into database
            db.insert(key.to_string(), &*task)
                .expect("could not insert task");
        }
    }
}
