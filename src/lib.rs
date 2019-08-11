mod diskv;

pub mod index;

// index:
pub use index::Index;

// diskv:
pub use diskv::Diskv;
pub use diskv::simple;
pub use diskv::new;
pub use diskv::Options;