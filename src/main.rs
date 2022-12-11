use sled::Db;

fn main() {
    let db: Db = sled::open("todos_db").expect("could not get database");

    db.insert("hello", "world").expect("could not insert");

    let first = db.first();

    if let Ok(Some((key, value))) = first {
        println!("first key: {}", String::from_utf8(key.to_vec()).unwrap());
        println!(
            "first value: {}",
            String::from_utf8(value.to_vec()).unwrap()
        );
    }
}
