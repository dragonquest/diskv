use std::fmt;

pub mod multiindex;
pub use multiindex::MultiIndex as MultiIndex;

pub trait Index {
    fn get(&self, key: &'static str) -> IndexResult;
    fn set(&self, key: &'static str, value: &[u8]) -> IndexResult;
    fn delete(&self, key: &'static str) -> IndexResult;
}

/// Index result
#[derive(Debug)]
pub enum IndexResult {
    Ok,
    NotFound,
    Found(Vec<u8>),
    Skipped,
    Error(String),
}

impl fmt::Display for IndexResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IndexResult::Ok => write!(f, "Ok"),
            IndexResult::NotFound => write!(f, "Key not found"),
            IndexResult::Found(data) => write!(f, "Found data (length={})", data.len()),
            IndexResult::Error(err_msg) => write!(f, "Error('{}')", err_msg),
            IndexResult::Skipped => write!(f, "Skipped"),
        }
    }
}
