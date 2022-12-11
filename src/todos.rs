use sled::Db;

pub fn print_todos(db: &Db) {
    println!("Your tasks:");
    db.iter().for_each(|todo_item| match todo_item {
        Ok((key, value)) => {
            let key = String::from_utf8(key.to_vec()).unwrap();
            let value = String::from_utf8(value.to_vec()).unwrap();
            println!("{}: {}", key, value);
        }
        Err(e) => println!("error: {}", e),
    });
}

pub fn add_todo(db: &Db, todo: String) {
    let key = db
        .iter()
        .map(|item| {
            let key = item.expect("could not get key").0;
            let key = String::from_utf8(key.to_vec()).expect("could not convert to string");
            let key = key.parse::<u64>().expect("could not parse key");
            key + 1
        })
        .last()
        .unwrap_or(1);

    // Insert task into database
    db.insert(key.to_string(), &*todo)
        .expect("could not insert task");
}

pub fn remove_todo(db: &Db, key: u64) -> Result<(), String> {
    match db.remove(key.to_string()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("could not remove task: {}", e)),
    }
}
