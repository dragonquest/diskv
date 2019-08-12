pub use super::Index;
pub use super::IndexResult;

/// MultiIndex is an Index that can hold more chained indices
pub struct MultiIndex {
    indices: Vec<Box<dyn Index>>,
}

impl MultiIndex {
    pub fn new(indices: Vec<Box<dyn Index>>) -> Box<dyn Index> {
        let mi = MultiIndex { indices: indices };

        Box::new(mi)
    }
}

impl Index for MultiIndex {
    /// get iterates through all indices and returns the first value found.
    /// If an error has occured then it returns IndexResult::Error.
    /// If nothing found it returns IndexResult::NotFound
    fn get(&mut self, key: &'static str) -> IndexResult {
        for index in &mut self.indices {
            let res = &mut index.get(key);
            if let IndexResult::Found(v) = res {
                return IndexResult::Found(v.to_vec());
            }

            if let IndexResult::Error(err_msg) = res {
                return IndexResult::Error(err_msg.to_string());
            }
        }

        IndexResult::NotFound
    }

    /// set(k,v) iterates through the registered indices and tries to
    /// set a value eagerly. if a set has succeeded then it will
    /// return with IndexResult::Ok, otherwise an IndexResult::Error will be
    /// returned. Should no value be set, then a IndexResult::Skipped will be returned.
    fn set(&mut self, key: &'static str, value: &[u8]) -> IndexResult {
        for index in &mut self.indices {
            let res = &mut index.set(key, value);
            if let IndexResult::Ok = res {
                return IndexResult::Ok;
            }

            if let IndexResult::Error(err_msg) = res {
                return IndexResult::Error(err_msg.to_string());
            }
        }

        IndexResult::Skipped
    }

    /// deletes() deletes a given key. if an error has occured it will
    /// return via IndexResult:Error(err_msg) otherwise it'll try to delete the entry.
    /// If no bad error has been returned, it assumes that it got deleted along the way.
    /// (It won't return on the first succeeded delete because maybe the key is registered in multi indices!?)
    fn delete(&mut self, key: &'static str) -> IndexResult {
        for index in &mut self.indices {
            let res = &mut index.delete(key);

            if let IndexResult::Error(err_msg) = res {
                return IndexResult::Error(err_msg.to_string());
            }
        }

        IndexResult::Ok
    }
}
