pub fn validate_cap(bfr_cap: usize, bfr_len: usize, otr_len: usize) -> Result<(), String> {
    if (bfr_len + otr_len) > bfr_cap {
        let errmsg = format!("Buffer capacity is {}, but trying to store {}", bfr_cap, (bfr_len+otr_len));
        return Err(errmsg)
    }
    Ok(())
}

pub fn validate_len(bfr_len: usize, idx: usize) -> Result<(), String> {
    if idx >= bfr_len {
        let errmsg = format!("Buffer length is {}, but trying to access index {}", bfr_len, idx);
        return Err(errmsg)
    }
    Ok(())
}