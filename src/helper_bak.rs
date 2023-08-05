
pub mod error_text {
    pub fn not_enough_capacity(buffer: usize, value: usize) -> String {
        format!("Capacity of buffer is {} but trying to store {}", buffer, value)
    }
    pub fn not_valid_index(len: usize, index: usize) -> String {
        format!("Buffer len is {} but trying to access index at {}", len, index)
    }
}

