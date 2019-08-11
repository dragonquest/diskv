use diskv;

use diskv::index::MultiIndex;

fn main() {
    // let db = diskv::simple("./db1").unwrap();

    let mi = MultiIndex::new(vec![]);

    let options = diskv::Options {
        base_dir: "./db1".to_string(),
        index: Some(mi),
    };

    let db = diskv::new(options).unwrap();

    db.write("name", "Susan".as_bytes()).unwrap();

    let x = db.read("name").unwrap();
    println!("Stored: {}", String::from_utf8_lossy(&x));
    db.delete("name").unwrap();

    println!("from main.rs");
}