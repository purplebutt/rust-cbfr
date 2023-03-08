use std::mem;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div, Deref};

use crate::helper::error_text as err_txt;


pub const MAX: usize = 8192;    // 8kb

/// 8 bytes buffer
/// # example:
/// ```
/// use cbfr::core::b8kb::B8KB;
///
/// let b = B8KB::from("some string");
/// assert_eq!(b.to_string(), "some string");
/// ```
pub struct B8KB { 
    arr: [u8; MAX],
    len: usize 
}

/// Create B8KB instance from &str
/// # example
/// ```
/// use cbfr::core::b8kb::B8KB;
///
/// let b: B8KB = "some string".into();
/// ```
/// # panic
/// Panic if "some string" len > cbfr::core::b8kb::MAX
///
impl From<&str> for B8KB {
    fn from(value: &str) -> Self {
        if value.len() <= MAX {
            let mut a = [0u8; MAX];
            for (i, &v) in value.as_bytes().iter().enumerate() {
                a[i] = v
            };
            Self {
                arr: a,
                len: value.len()
            }
        }
        else {
            panic!("{}", err_txt::not_enough_capacity(MAX, value.len()))
        }
    } 
}

impl Display for B8KB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contains_value = &self.arr[0..self.len];
        let text: &str;
        unsafe {
            text = std::str::from_utf8_unchecked(contains_value);
        }
        write!(f, "{}", text)
    }
}

impl Clone for B8KB {
    fn clone(&self) -> Self {
        Self { 
            arr: self.arr.clone(), 
            len: self.len 
        }
    }
}

impl PartialEq for B8KB {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false
        } 
        else {
            for (idx, c) in self.arr.iter().enumerate() {
                if *c != other.arr[idx] {
                    return false;
                }
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        if self.len != other.len {
            return true
        } 
        else {
            for (idx, c) in self.arr.iter().enumerate() {
                if *c != other.arr[idx] {
                    return true;
                }
            }
        }
        false
    }
}

impl PartialOrd for B8KB {
    fn lt(&self, other: &Self) -> bool { self.checksum() < other.checksum() }
    fn le(&self, other: &Self) -> bool { self.checksum() <= other.checksum() }
    fn gt(&self, other: &Self) -> bool { self.checksum() > other.checksum() }
    fn ge(&self, other: &Self) -> bool { self.checksum() >= other.checksum() }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for B8KB {}

impl Ord for B8KB {
    fn max(self, other: Self) -> Self
        where Self: Sized 
    {
        if self.checksum() > other.checksum() {self}
        else { other }
    }
    fn min(self, other: Self) -> Self
        where Self: Sized 
    {
        if self.checksum() < other.checksum() {self}
        else { other }
    }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.checksum() < other.checksum() { std::cmp::Ordering::Less }
        else if self.checksum() > other.checksum() { std::cmp::Ordering::Greater }
        else { std::cmp::Ordering::Equal } 
    }
    fn clamp(self, min: Self, max: Self) -> Self
        where Self: Sized,
    {
        if self.checksum() > max.checksum() {max}
        else if self.checksum() < min.checksum() {min}
        else {self}
    }
}

impl Add for B8KB {
    type Output = usize;
    fn add(self, rhs: Self) -> Self::Output {
        self.checksum() + rhs.checksum()
    }
}

impl Sub for B8KB {
    type Output = usize;
    fn sub(self, rhs: Self) -> Self::Output {
        self.checksum() - rhs.checksum()
    }
}

impl Mul for B8KB {
    type Output = usize;
    fn mul(self, rhs: Self) -> Self::Output {
        self.checksum() * rhs.checksum()
    }
}

impl Div for B8KB {
    type Output = f64;
    fn div(self, rhs: Self) -> Self::Output {
        self.checksum() as f64 / rhs.checksum() as f64
    }
}

impl Deref for B8KB {
    type Target = [u8; MAX];
    fn deref(&self) -> &Self::Target {
        &self.arr
    }
}


impl B8KB {
   /// create new buffer with capacity of b8kb::MAX
   /// # example
   /// ```
   /// use cbfr::core::b8kb::B8KB;
   /// let b = B8KB::new();
   /// assert_eq!("", b.to_string());
   /// ```
   ///
    pub fn new() -> Self {
        Self { arr: [0u8; MAX], len: 0 }
    }

    /// return buffer as &str
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    /// let b: B8KB = "some string".into();
    /// assert_eq!("some string", b.as_str());
    /// ```
    ///
    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(&self.arr[0..self.len])
        }
    }

    /// return len of buffer
    /// len is different with capacity. Capacity is a constant value
    /// (b8kb::MAX), which is the maximum value of data (in bytes) a buffer can hold
    /// while len is the current len (in bytes) of data inside a buffer
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    /// let b: B8KB = "some string".into();
    /// assert_eq!(11, b.len());
    /// ```
    ///
    pub fn len(&self) -> usize { self.len }

    /// get the checksum or total of buffer value in accordance
    /// to it's utf code
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    /// let b: B8KB = "Aa".into();
    ///
    /// assert_eq!((65+65+32), b.checksum());
    /// ```
    pub fn checksum(&self) -> usize {
        let mut result = 0;
        for c in self.arr.iter() {
            result += *c as usize;
        }
        result
    }

    /// get the last char inside a buffer
    /// if char is empty, this function return '\0'
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    /// let b: B8KB = "some string".into();
    /// assert_eq!('g', b.last());
    ///
    /// let c = B8KB::new();
    /// assert_eq!('\0', c.last());
    /// ```
    ///
    pub fn last(&self) -> char {
        if self.len > 0 {
            self.arr[self.len-1].into()
        }
        else {
            '\0'
        }
    }

    /// return the capacity of buffer, that is the maximum
    /// bytes this buffer can hold.This function simply return
    /// b8kb::MAX constant value.
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    /// use cbfr::core::b8kb::MAX;
    ///
    /// let b = B8KB::new();
    /// assert_eq!(MAX, b.capacity());
    /// ```
    ///
    pub fn capacity(&self) -> usize { MAX }

    /// clear all data inside a buffer, causing all data
    /// to be revert back to buffer default value that is '\0'
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "some string".into();
    /// b.clear();
    /// assert_eq!(0, b.len());
    /// assert_eq!("", b.as_str());
    /// assert_eq!("", b.to_string());
    /// ```
    /// 
    pub fn clear(&mut self) {
         self.arr = [0u8; MAX];
         self.len = 0;
    }

    /// swap buffer A with buffer B naively without any error checking
    /// offer performance (no error checking logic) but with a risk
    fn swap_unchecked(&mut self, mut other: Self) {
        mem::swap(&mut self.arr, &mut other.arr);
        for i in 0..self.len {
            self.arr[other.len+i] = other.arr[i]
        }
        self.len += other.len;
    }

    /// prepend current buffer with another buffer
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut ba: B8KB = "coding".into();
    /// let bb: B8KB = "I love ".into();
    ///
    /// ba.prepend(bb);
    /// assert_eq!("I love coding", ba.as_str());
    /// assert_eq!(13, ba.len());
    /// ```
    ///
    pub fn prepend(&mut self, other: Self) -> Result<(), String> {
        let total_len = self.len + other.len;
        if total_len <= MAX {
            self.swap_unchecked(other);
            Ok(())
        }
        else {
            Err(err_txt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// prepend buffer with &str
    /// # example:
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "coding".into();
    /// let some_str = "I love ";
    ///
    /// b.prepend_str(some_str);
    /// assert_eq!("I love coding", b.as_str());
    /// assert_eq!(13, b.len());
    /// ```
    ///
    pub fn prepend_str(&mut self, text: &str) -> Result<(), String> {
        let total_len = self.len + text.len();
        let other: Self = text.into();
        if total_len <= MAX {
            self.swap_unchecked(other);
            Ok(())
        }
        else {
            Err(err_txt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// append current buffer with another buffer
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut ba: B8KB = "I love".into();
    /// let bb: B8KB = " coding".into();
    ///
    /// ba.append(bb);
    /// assert_eq!("I love coding", ba.as_str());
    /// assert_eq!(13, ba.len());
    /// ```
    ///
    pub fn append(&mut self, other: Self) -> Result<(), String> {
        let total_len = self.len + other.len;
        if total_len <= MAX {
            for i in 0..(other.len) {
                self.arr[self.len+i] = other.arr[i]
            }
            self.len += other.len;
            Ok(())
        }
        else {
            Err(err_txt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// append current buffer with &str
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "I love".into();
    /// let some_string = " coding";
    ///
    /// b.append_str(some_string);
    /// assert_eq!("I love coding", b.as_str());
    /// assert_eq!(13, b.len());
    /// ```
    ///
    pub fn append_str(&mut self, text: &str) -> Result<(), String> {
        let total_len = self.len + text.len();
        if total_len <= MAX {
            for i in 0..text.len() {
                self.arr[self.len+i] = text.as_bytes()[i]
            }
            self.len += text.len();
            Ok(())
        }
        else {
            Err(err_txt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// append current buffer with &str
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "Happy codin".into();
    ///
    /// b.append_ch('g');
    /// assert_eq!("Happy coding", b.as_str());
    /// assert_eq!(12, b.len());
    /// ```
    ///
    pub fn append_ch(&mut self, c: char) -> Result<(), String> {
        let total_len = self.len + c.len_utf8();
        if total_len <= MAX {
            for i in 0..c.len_utf8() {
                self.arr[self.len+i] = c as u8
            }
            self.len += c.len_utf8();
            Ok(())
        }
        else {
            Err(err_txt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// shift value to right, leave original value
    /// this function will expand the buffer value len by 1
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "Amazing".into();
    ///
    /// b.rshift(6).unwrap();
    /// assert_eq!("Amazingg", b.as_str());
    /// assert_eq!(8, b.len());
    /// ```
    ///
    pub fn rshift(&mut self, pos: usize) -> Result<(), String> {
        if pos < self.len && self.len < MAX {
            let mut idx = self.len;
            while idx > pos  {
                self.arr[idx] = self.arr[idx-1];
                idx -= 1;
            }
            self.len += 1;
            Ok(())
        }
        else {
            Err(err_txt::not_valid_index(self.len, pos))
        }
    }

    /// shift value to left
    /// this function will shrink buffer len by 1
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "Amazing".into();
    ///
    /// b.lshift(0).unwrap();
    /// assert_eq!("mazing", b.as_str());
    /// assert_eq!(6, b.len());
    /// ```
    ///
    pub fn lshift(&mut self, pos: usize) -> Result<(), String> {
        if pos < self.len {
            for i in pos..self.len {
                self.arr[i] = self.arr[i+1]
            }
            self.arr[self.len] = 0u8;
            self.len -= 1;
            Ok(())
        }
        else {
            Err(err_txt::not_valid_index(self.len, pos))
        }
    }

    /// take and remove a value from buffer
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "AmazZing".into();
    ///
    /// let c = b.take(4).unwrap();
    /// assert_eq!('Z', c as char);
    /// assert_eq!("Amazing", b.as_str());
    /// assert_eq!(7, b.len());
    /// ```
    ///
    pub fn take(&mut self, pos: usize) -> Option<u8> {
        if pos < self.len {
            let result = self.arr[pos];
            self.lshift(pos).unwrap();
            Some(result)
        }
        else { None }
    }

    /// insert with another buffer at a given position
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b1: B8KB = "Amng".into();
    /// let mut b2: B8KB = "azi".into();
    ///
    /// b1.insert(b2, 2).unwrap();
    /// assert_eq!("Amazing", b1.as_str());
    /// assert_eq!(7, b1.len());
    /// ```
    ///
    pub fn insert(&mut self, other: Self, pos: usize) -> Result<(), String> {
        let total_len = self.len + other.len;
        if total_len <= MAX {
            let mut idx = pos;
            for i in 0..other.len {
                self.rshift(idx)?;
                self.arr[idx] = other.arr[i];
                idx += 1;
            }
            Ok(())
        }
        else {
            Err(err_txt::not_enough_capacity(MAX, total_len))
        }
    }
    
    /// insert with char at a given position
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "Amaing".into();
    ///
    /// b.insert_ch('Z', 3).unwrap();
    /// assert_eq!("AmaZing", b.as_str());
    /// assert_eq!(7, b.len());
    /// ```
    ///
    pub fn insert_ch(&mut self, c: char, pos: usize) -> Result<(), String> {
        if self.len < MAX {
            self.rshift(pos)?;
            self.arr[pos] = c as u8;
            Ok(())
        }
        else {
            Err(err_txt::not_enough_capacity(MAX, self.len + 1))
        }
    }
    
    /// Reverse order of items in buffer
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "12345".into();
    ///
    /// b.reverse();
    /// assert_eq!("54321", b.as_str());
    /// assert_eq!(5, b.len());
    /// ```
    ///
    pub fn reverse(&mut self) {
        let mid = self.len/2;
        let mut idx = (0usize, self.len-1);
        while idx.0 < mid {
            let temp = self.arr[idx.0];
            self.arr[idx.0] = self.arr[idx.1];
            self.arr[idx.1] = temp;
            idx.0 += 1;
            idx.1 -= 1;
        }
    }

    /// sort items in buffer
    /// takes O(log n) time complexity
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "cgahb".into();
    ///
    /// b.sort();
    /// assert_eq!("abcgh", b.as_str());
    /// assert_eq!(5, b.len());
    /// ```
    ///
    pub fn sort(&mut self) {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 0..(self.len-1) {
                if self.arr[i+1] < self.arr[i] {
                    let temp = self.arr[i];
                    self.arr[i] = self.arr[i+1];
                    self.arr[i+1] = temp;
                    sorted = false;
                }
            }
        }
    }

    /// sort items in buffer descending
    /// takes O(log n) time complexity
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "cgahb".into();
    ///
    /// b.sort_desc();
    /// assert_eq!("hgcba", b.as_str());
    /// assert_eq!(5, b.len());
    /// ```
    ///
    pub fn sort_desc(&mut self) {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 0..(self.len-1) {
                if self.arr[i+1] > self.arr[i] {
                    let temp = self.arr[i];
                    self.arr[i] = self.arr[i+1];
                    self.arr[i+1] = temp;
                    sorted = false;
                }
            }
        }
    }

    /// trim space on left
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "  L ove".into();
    ///
    /// b.ltrim();
    /// assert_eq!("L ove", b.as_str());
    /// assert_eq!(5, b.len());
    /// ```
    ///
    pub fn ltrim(&mut self) {
        let mut idx = self.len;
        while self.arr[0] == ' ' as u8 && idx > 1 {
            self.lshift(0).unwrap();
            idx -= 1;
        }
    }

    /// trim space on right
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "Lov e  ".into();
    ///
    /// b.rtrim();
    /// assert_eq!("Lov e", b.as_str());
    /// assert_eq!(5, b.len());
    /// ```
    ///
    pub fn rtrim(&mut self) {
        while self.arr[self.len-1] == ' ' as u8 && self.len > 1 {
            self.arr[self.len-1] = 0u8;
            self.len -= 1;
        }
    }

    /// trim space
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = " Lov e  ".into();
    ///
    /// b.trim();
    /// assert_eq!("Lov e", b.as_str());
    /// assert_eq!(5, b.len());
    /// ```
    ///
    pub fn trim(&mut self) {
        self.ltrim();
        self.rtrim();
    }

    /// to lower
    /// convert value to lowercase
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "LoVE".into();
    ///
    /// b.to_lower();
    /// assert_eq!("love", b.as_str());
    /// assert_eq!(4, b.len());
    /// ```
    ///
    ///
    pub fn to_lower(&mut self) {
        for c in self.arr.iter_mut() {
            if *c <= 90 && *c >= 65 {
                *c = *c + 32;
            }
        }
    }

    /// to upper
    /// convert value to uppercase
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "Love".into();
    ///
    /// b.to_upper();
    /// assert_eq!("LOVE", b.as_str());
    /// assert_eq!(4, b.len());
    /// ```
    ///
    ///
    pub fn to_upper(&mut self) {
        for c in self.arr.iter_mut() {
            if *c >= 97 && *c <= 122 {
                *c = *c - 32;
            }
        }
    }

    /// to title
    /// convert all value to lowercase except for the first letter
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "lOVE".into();
    ///
    /// b.to_title();
    /// assert_eq!("Love", b.as_str());
    /// assert_eq!(4, b.len());
    /// ```
    ///
    ///
    pub fn to_title(&mut self) {
        self.to_lower();
        self.arr[0] -= 32;
    }

    /// to proper
    /// convert all value to lowercase but uppercase for every first letter of each words
    /// # example
    /// ```
    /// use cbfr::core::b8kb::B8KB;
    ///
    /// let mut b: B8KB = "damN i loVe iNdoNesIA".into();
    ///
    /// b.to_proper();
    /// assert_eq!("Damn I Love Indonesia", b.as_str());
    /// assert_eq!(21, b.len());
    /// ```
    ///
    pub fn to_proper(&mut self) {
        let mut change_next = false;
        self.to_title();
        for (idx, c) in self.arr.iter_mut().enumerate()  {
    
            if change_next && *c != ' ' as u8 {
                *c = *c - 32;
                change_next = false
            }

            if idx < self.len-1 && *c == ' ' as u8 {
                change_next = true;
            }
        }
    }

}

