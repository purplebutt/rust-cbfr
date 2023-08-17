use std::fmt::Display;
use std::error::Error;

#[doc = "hidden"]
#[derive(Debug, Default)]
struct ErrorBase {
    buffer: usize,
    value: usize,
    len: usize,
    index: usize
}

#[doc = "hidden"]
#[derive(Debug)]
pub struct NotEnoughCapacity(ErrorBase);
impl Error for NotEnoughCapacity {}
impl Display for NotEnoughCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = 
            format!("Capacity of buffer is {} but trying to store {}", 
                self.0.buffer, self.0.value);
        write!(f, "NotEnoughCapacity: \"{}\"", err_msg)
    }
}
impl NotEnoughCapacity {
    pub fn throw(buffer: usize, value: usize) -> Self {
        Self(ErrorBase { buffer, value, ..Default::default() })
    }
}

#[doc = "hidden"]
#[derive(Debug)]
pub struct InvalidIndex(ErrorBase);
impl Error for InvalidIndex {}
impl Display for InvalidIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = 
            format!("Buffer len is {} but trying to access index at {}", 
                self.0.len, self.0.index);
        write!(f, "InvalidIndex: \"{}\"", err_msg)
    }
}
impl InvalidIndex {
    pub fn throw(len: usize, index: usize) -> Self {
        Self(ErrorBase { len, index, ..Default::default() })
    }
}

impl From<InvalidIndex> for NotEnoughCapacity {
    fn from(value: InvalidIndex) -> Self {
        Self(ErrorBase { buffer: value.0.len, value: value.0.index, ..Default::default() })
    }
}
impl From<NotEnoughCapacity> for InvalidIndex {
    fn from(value: NotEnoughCapacity) -> Self {
        Self(ErrorBase { len: value.0.buffer, index: value.0.value, ..Default::default() })
    }
}
