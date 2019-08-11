use std::error::Error;
use std::fs::create_dir_all;
use string_error::*;
use std::path::*;
use std::io::Write;

use crate::Index;
use crate::index::IndexResult;

pub type ErrorResult<T> = Result<T, Box<dyn Error>>;

pub struct Options {
    pub base_dir: String,
    pub index: Option<Box<dyn Index>>,
}

pub struct Diskv {
    options: Options,
}

impl Diskv {
    pub fn write_str(&mut self, key: &'static str, value: &'static str) -> ErrorResult<()> {
        self.write(key, value.as_bytes())
    }

    pub fn write_string(&mut self, key: &'static str, value: String) -> ErrorResult<()> {
        self.write(key, value.as_bytes())
    }

    pub fn write(&mut self, key: &'static str, value: &[u8]) -> ErrorResult<()> {
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

        if let Some(index) = &mut self.options.index {
            let resp = index.set(key, value);

            if let IndexResult::Error(err_msg) = resp {
                let err = string_error::new_err(err_msg.as_ref());
                return Err(err);
            }
        }

        return Ok(());
    }

    pub fn read(&mut self, key: &'static str) -> ErrorResult<Vec<u8>> {

        if let Some(index) = &mut self.options.index {
            let resp = index.get(key);

            if let IndexResult::Error(err_msg) = resp {
                let err = string_error::new_err(err_msg.as_ref());
                return Err(err);
            } else if let IndexResult::Found(v) = resp {
                return Ok(v);
            }
        }

        let key_path = Path::new(&self.options.base_dir).join(key);

        match std::fs::read(key_path) {
            Err(e) => Err(Box::new(e)),
            Ok(data) => Ok(data),
        }
    }

    pub fn delete(&mut self, key: &'static str) -> ErrorResult<()> {
        if let Some(index) = &mut self.options.index {
            let resp = index.delete(key);

            if let IndexResult::Error(err_msg) = resp {
                let err = string_error::new_err(err_msg.as_ref());
                return Err(err);
            }
        }

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