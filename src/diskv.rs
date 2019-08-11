use std::error::Error;
use std::fs::create_dir_all;
use string_error::*;
use std::path::*;
use std::io::Write;
use std::fs::read;

use crate::Index;

pub type ErrorResult<T> = Result<T, Box<dyn Error>>;

pub struct Options {
    pub base_dir: String,
    pub index: Option<Box<dyn Index>>,
}

pub struct Diskv {
    options: Options,
}

impl Diskv {
    pub fn write_str(&self, key: &'static str, value: &'static str) -> ErrorResult<()> {
        self.write(key, value.as_bytes())
    }

    pub fn write_string(&self, key: &'static str, value: String) -> ErrorResult<()> {
        self.write(key, value.as_bytes())
    }

    pub fn write(&self, key: &'static str, value: &[u8]) -> ErrorResult<()> {
        let key_path = Path::new(&self.options.base_dir).join(key);

        let file = std::fs::File::create(key_path);

        if let Err(e) = file {
            return Err(Box::new(e));
        }
        let file = file.unwrap();

        let mut buf_file = std::io::BufWriter::new(file);
        if let Err(e) = buf_file.write(value) {
            return Err(Box::new(e));
        }

        return Ok(());
    }

    pub fn read(&self, key: &'static str) -> ErrorResult<Vec<u8>> {
        let key_path = Path::new(&self.options.base_dir).join(key);

        match read(key_path) {
            Err(e) => Err(Box::new(e)),
            Ok(data) => Ok(data),
        }
    }

    pub fn delete(&self, key: &'static str) -> ErrorResult<()> {
        let key_path = Path::new(&self.options.base_dir).join(key);

        if !key_path.exists() {
            return Ok(());
        }

        match std::fs::remove_file(key_path) {
            Err(e) => Err(Box::new(e)),
            Ok(_) => Ok(()),
        }
    }
}

pub fn new(options: Options) -> ErrorResult<Diskv> {
    let db = Diskv { options };
    
    let created_dir = create_dir_all(&db.options.base_dir);

    if let Err(e) = created_dir {
        return Err(new_err(&format!("Failed to create '{}': {}", &db.options.base_dir, e).to_string()));
    }

    Ok(db)
}

pub fn simple(base_dir: &'static str) -> ErrorResult<Diskv> {
    let db = Diskv {
        options: Options {
            base_dir: base_dir.to_string(),
            index: None,
        },
    };

    let created_dir = create_dir_all(&db.options.base_dir);

    if let Err(e) = created_dir {
        return Err(new_err(&format!("Failed to create '{}': {}", &db.options.base_dir, e).to_string()));
    }

    Ok(db)
}