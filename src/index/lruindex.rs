pub use super::Index;
pub use super::IndexResult;

use lru_cache::LruCache;
use std::vec::*;

pub struct LruIndex {
    cache: LruCache<&'static str, Vec<u8>>,
}

impl LruIndex {
    pub fn new(capacity: usize) -> impl Index {
        let cache = LruCache::<&'static str, Vec<u8>>::new(capacity);
        let lru = LruIndex { cache: cache };

        lru
    }
}

impl Index for LruIndex {
    fn get(&mut self, key: &'static str) -> IndexResult {
        let found = self.cache.get_mut(key);

        if let Some(v) = found {
            return IndexResult::Found(v.to_vec());
        }

        IndexResult::NotFound
    }

    fn set(&mut self, key: &'static str, value: &[u8]) -> IndexResult {
        // insert returns the old value. We are not interested in that
        // so we ignore the return value deliberately:
        self.cache.insert(key, value.to_vec());

        IndexResult::Ok
    }

    fn delete(&mut self, key: &'static str) -> IndexResult {
        // remove returns the value removed. We are not interested in that
        // so we ignore the return value deliberately:
        self.cache.remove(key);
        IndexResult::Ok
    }
}
