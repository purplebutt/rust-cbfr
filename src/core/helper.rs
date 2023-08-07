use crate::core::errors::NotEnoughCapacity;


/// helper function for Display trait implementation
#[doc = "hidden"]
pub fn fmt(len: &usize, arr: &[u8], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let contain_values = &arr[0..arr.len()];
    let text: &str;
    unsafe {
        text = std::str::from_utf8_unchecked(contain_values);
    }
    write!(f, "{}", &text[0..*len])
}

/// helper function for From trait implementation
#[doc = "hidden"]
pub fn from(value: &str, arr: &mut [u8]) {
    if value.len() <= arr.len() {
        for (i, &v) in value.as_bytes().iter().enumerate() {
            arr[i] = v
        };
    } else {
        panic!("{}", NotEnoughCapacity::throw(arr.len(), value.len()));
    }
} 

#[doc = "hidden"]
pub fn from_slice(value: &[u8], arr: &mut [u8]) {
    if value.len() <= arr.len() {
        for (i, &v) in value.iter().enumerate() {
            arr[i] = v;
        }
    } else {
        panic!("{}", NotEnoughCapacity::throw(arr.len(), value.len()));
    }
} 

#[doc = "hidden"]
/// helper function for partialEQ trait implementation
pub fn eq(lena: &usize, arra: &[u8], lenb: &usize, arrb: &[u8]) -> bool {
    if lena != lenb {
        return false;
    } else {
        for (i, c) in arra.iter().enumerate() {
            if *c != arrb[i] {
                return false;
            }
        }
    }
    true
}

#[doc = "hidden"]
pub fn ne(lena: &usize, arra: &[u8], lenb: &usize, arrb: &[u8]) -> bool {
    if lena != lenb {
        return true;
    } else {
        for (i, c) in arra.iter().enumerate() {
            if *c != arrb[i] {
                return true;
            }
        }
    }
    false
}

#[doc = "hidden"]
pub fn append_ch_unchecked(len: &mut usize, arr: &mut [u8], c: char) {
    arr[*len] = c as u8;
    *len += 1;
} 

#[doc = "hidden"]
pub fn append_ch(len: &mut usize, arr: &mut [u8], c: char) -> Result<(), NotEnoughCapacity> {
    let total_len = *len + c.len_utf8();
    if total_len < arr.len() {
        for i in 0..c.len_utf8() {
            arr[*len+i] = c as u8;
        }
        *len += c.len_utf8();
        Ok(())
    } else {
        Err(NotEnoughCapacity::throw(arr.len(), total_len))
    }
}

#[doc = "hidden"]
pub fn checksum(arr: &[u8]) -> usize {
    let mut result = 0;
    for i in arr.iter() {
        result += *i as usize;
    }
    result
}
