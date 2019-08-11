use diskv;

use diskv::index::MultiIndex;
use diskv::index::LruIndex;

fn main() {
    // let db = diskv::simple("./db1").unwrap();

    let lru = LruIndex::new(2000);
    let mi = MultiIndex::new(vec![lru]);

    let options = diskv::Options {
        base_dir: "./db1".to_string(),
        index: Some(mi),
    };

    let mut db = diskv::new(options).unwrap();

    db.write("name", "Susan".as_bytes()).unwrap();
    db.write("name1", "Susan".as_bytes()).unwrap();
    db.write("name2", "Susan".as_bytes()).unwrap();
    db.write("name3", "Susan".as_bytes()).unwrap();
    db.write("name4", "Susan".as_bytes()).unwrap();

    let x = db.read("name").unwrap();
    println!("Stored: {}", String::from_utf8_lossy(&x));
    db.delete("name").unwrap();

    db.read("name4").unwrap();

    println!("from main.rs");
}