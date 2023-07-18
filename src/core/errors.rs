use std::fmt::Display;
use std::error::Error;


#[derive(Debug)]
struct ErrorBase {
    buffer: usize,
    value: usize
}

#[derive(Debug)]
struct NotEnoughCapacity(ErrorBase);
impl Error for NotEnoughCapacity {}
impl Display for NotEnoughCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = 
            format!("Capacity of buffer is {} but trying to store {}", 
                self.0.buffer, self.0.value);
        write!(f, "NotEnoughCapacity error: \"{}\"", err_msg)
    }
}