use kvstore::Database;
use std::io;

fn main() -> ! {
    println!("---------------Database Name: KVstore------------");
    let mut db = Database::new();
    loop {
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Bad Input");
        let mut query = db.parse(&input).unwrap();
        if let Some(v) = db.exec(&mut query) {
            println!("Result>{}",v);
        } else {
            println!("------------");
        }
    }
}