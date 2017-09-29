
#![feature(libc)]

extern crate libc;

mod native;

pub type Result<T> = std::result::Result<T, SkeinError>;

pub fn hash(hashbitlen: i32, data: &[u8], hashval: &mut [u8]) -> Result<()> {
    match unsafe { native::skein_hash(hashbitlen, data.as_ptr(), data.len() as u64 * 8, hashval.as_mut_ptr()) } {
        0 => Ok(()),
        e => Err(SkeinError::from(e)),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SkeinError {
    /// Generic failure state
    Fail,
    /// `hashbitlen` or `hash()` incorrect
    BadHashbitlen,
}

impl From<i32> for SkeinError {
    /// Passing incorrect error values yields unspecified behaviour.
    fn from(i: i32) -> Self {
        match i {
            0 => panic!("Not an error"),
            1 => SkeinError::Fail,
            2 => SkeinError::BadHashbitlen,
            _ => panic!("Incorrect error number"),
        }
    }
}
