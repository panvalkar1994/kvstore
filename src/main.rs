use kvstore::{Database, Query};
use std::io;

fn main() -> ! {
    println!("---------------Database Name: KVstore------------");
    let mut db = Database::new();
    loop {
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Bad Input");
        let query = Query::new(&input).unwrap();
        if let Some(v) = query.exec(&mut db) {
            println!("Result>{}",v);
        } else {
            println!("------------");
        }
    }
}