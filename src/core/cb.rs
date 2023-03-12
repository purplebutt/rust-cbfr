use std::panic::RefUnwindSafe;
use std::{mem, result};
use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Div, Deref};


use crate::core::helper as helper;
use crate::helper::error_text as errtxt;


/// buffer
/// # example:
/// ```
/// use cbfr::core::cb::BFRDYN;
///
/// let b = BFRDYN::<256>::from("some string");
/// assert_eq!(b.to_string(), "some string");
/// ```
pub struct BFRDYN<const CAPACITY: usize> {
    arr: [u8; CAPACITY],
    len: usize
}

// Display Trait
impl<const CAPACITY: usize> Display for BFRDYN<CAPACITY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { helper::fmt(&self.len, &self.arr, f) }
}

/// Create buffer instance from &str
/// # example
/// ```
/// use cbfr::core::cb::BFRDYN;
///
/// let b: BFRDYN<256> = "some string".into();
/// ```
/// # panic
/// Panic if "some string" len > 256
///
impl<const CAPACITY: usize> From<&str> for BFRDYN<CAPACITY> {
    fn from(value: &str) -> Self {
        let mut arr = [0u8; CAPACITY];
        helper::from(value, &mut arr);
        Self { arr, len: value.len() }
    } 
}

/// clone trait
impl<const CAPACITY: usize> Clone for BFRDYN<CAPACITY> {
    fn clone(&self) -> Self { Self { arr: self.arr.clone(), len: self.len.clone() } }
}

/// partialEQ trait
/// # example
/// ```
/// use cbfr::core::cb::BFRDYN;
///
/// let a: BFRDYN<256> = "some string".into();
/// let mut b: BFRDYN<256> = "some string".into();
///
/// assert_eq!(true, (a==b));
///
/// b.append_ch('x');
/// assert_eq!(false, (a==b));
///
/// ```
///
impl<const CAPACITY: usize> PartialEq for BFRDYN<CAPACITY> {
    fn eq(&self, other: &Self) -> bool {
        helper::eq(&self.len, &self.arr, &other.len, &other.arr)
    } 
    fn ne(&self, other: &Self) -> bool {
        helper::ne(&self.len, &self.arr, &other.len, &other.arr)
    }
}

/// partialOrd trait
/// # example
/// ```
/// use cbfr::core::cb::BFRDYN;
///
/// let a: BFRDYN<256> = "string".into();
/// let mut b: BFRDYN<256> = "some string".into();
///
/// assert_eq!(true, (a<b));
/// assert_eq!(true, (b>=a));
///
/// ```
///
impl<const CAPACITY: usize> PartialOrd for BFRDYN<CAPACITY> {
    fn lt(&self, other: &Self) -> bool { self.checksum() < other.checksum() }
    fn gt(&self, other: &Self) -> bool { self.checksum() > other.checksum() }
    fn le(&self, other: &Self) -> bool { self.checksum() <= other.checksum() }
    fn ge(&self, other: &Self) -> bool { self.checksum() >= other.checksum() }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

impl<const CAPACITY: usize> Eq for BFRDYN<CAPACITY> {}
impl<const CAPACITY: usize> Ord for BFRDYN<CAPACITY> {
    fn max(self, other: Self) -> Self
        where Self: Sized 
    {
        if self.checksum() > other.checksum() {
            self
        } 
        else {
            other
        }
    }
    fn min(self, other: Self) -> Self
        where Self: Sized 
    {
        if self.checksum() < other.checksum() {
            self
        } 
        else {
            other
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where Self: Sized 
    {
        if self.checksum() < min.checksum() {
            min
        } 
        else if self.checksum() > max.checksum() {
            max
        }
        else {
            self
        }
    }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.checksum() < other.checksum() {
            std::cmp::Ordering::Less
        }
        else if self.checksum() == other.checksum() {
            std::cmp::Ordering::Equal
        }
        else {
            std::cmp::Ordering::Greater
        }
    }  
}

impl<const CAPACITY: usize> Add for BFRDYN<CAPACITY> {
    type Output = usize;
    fn add(self, rhs: Self) -> Self::Output {
        self.checksum() + rhs.checksum()
    }
}

impl<const CAPACITY: usize> Sub for BFRDYN<CAPACITY> {
    type Output = usize;
    fn sub(self, rhs: Self) -> Self::Output {
        self.checksum() - rhs.checksum()    
    }
}

impl<const CAPACITY: usize> Mul for BFRDYN<CAPACITY> {
    type Output = usize;
    fn mul(self, rhs: Self) -> Self::Output {
        self.checksum() * rhs.checksum()
    }
}

impl<const CAPACITY: usize> Div for BFRDYN<CAPACITY> {
    type Output = f64;
    fn div(self, rhs: Self) -> Self::Output {
        self.checksum() as f64 / rhs.checksum() as f64
    }
}

// deref trait probably will expose self.arr to be able to access
// publicly. So it's better to not implement Deref trait
// impl<const CAPACITY: usize> Deref for BFRDYN<CAPACITY> {
//     type Target = [u8; CAPACITY];
//     fn deref(&self) -> &Self::Target {
//         &self.arr
//     } 
// }

// non trait implementations
impl<const CAPACITY: usize> BFRDYN<CAPACITY> {
    /// create new buffer with generic constant capacity
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let b = BFRDYN::<256>::new();
    /// assert_eq!("", b.to_string());
    /// assert_eq!(0, b.len());
    /// assert_eq!(256, b.capacity());
    /// ```
    ///
    pub fn new() -> Self { Self { arr: [0u8; CAPACITY], len: 0 } }

    /// return buffer as &str
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let b: BFRDYN<256> = "some string".into();
    /// assert_eq!("some string", b.as_str());
    /// ```
    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(&self.arr[0..self.len])
        }
    }

    /// get buffer capacity
    pub fn capacity(&self) -> usize { self.arr.len() }

    /// get buffer len
    pub fn len(&self) -> usize { self.len }

    /// perform checksum to all bytes data inside buffer
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let b: BFRDYN<125> = "Aa".into();
    ///
    /// assert_eq!((65+65+32), b.checksum());
    ///
    /// ```
    ///
    pub fn checksum(&self) -> usize {
        let mut result = 0;
        for c in self.arr.iter() {
            result += *c as usize;
        }
        result
    }

    /// get the last char inside a buffer
    /// if char is empty, this function return '\0'
    /// #example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let b: BFRDYN<256> = "some string".into();
    /// assert_eq!('g', b.last());
    ///
    /// let c = BFRDYN::<125>::new();
    /// assert_eq!('\0', c.last());
    /// ```
    ///
    pub fn last(&self) -> char {
        if self.len > 0 {
            self.arr[self.len-1].into()
        }
        else { '\0' }
    }

    /// clear all data inside a buffer, causing all data to be
    /// revert back to buffer default value that is '\0' or 0u8
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<125> = "some string".into();
    /// b.clear();
    /// assert_eq!(0, b.len());
    /// assert_eq!("", b.as_str());
    /// assert_eq!("", b.to_string());
    /// ```
    ///
    pub fn clear(&mut self) {
        self.arr = [0u8; CAPACITY];
        self.len = 0;
    }

    /// prepend buffer self with other without any error checking
    /// offer performance boost (no error checking logic) but with a risk
    /// after prepend, buffer self will contain buffer other old value +
    /// buffer self old value and buffer other will contain old
    /// buffer self value
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut a: BFRDYN<125> = "I love ".into();
    /// let mut b: BFRDYN<125> = "Indonesia".into();
    ///
    /// assert_eq!("I love ", a.to_string());
    /// assert_eq!("Indonesia", b.to_string());
    ///
    /// b.prepend_unchecked(&mut a);
    ///
    /// assert_eq!("Indonesia", a.to_string());
    /// assert_eq!("I love Indonesia", b.to_string());
    /// assert_eq!(9, a.len());
    /// assert_eq!(16, b.len());
    /// ```
    ///
    pub fn prepend_unchecked(&mut self, other: &mut Self) {
        mem::swap(&mut self.arr, &mut other.arr);
        for i in 0..self.len {
            self.arr[other.len+i] = other.arr[i]
        }
        // swap len
        let self_len_temp = self.len;
        self.len += other.len;
        other.len = self_len_temp;
    }

    /// prepend current buffer with another buffer
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    ///
    /// let mut a: BFRDYN<256> = "coding".into();
    /// let b: BFRDYN<256> = "I love ".into();
    ///
    /// a.prepend(b);
    /// assert_eq!("I love coding", a.as_str());
    /// assert_eq!(13, a.len());
    /// ```
    ///
    pub fn prepend(&mut self, mut other: Self) -> Result<(), String> {
        let total_len = self.len + other.len;
        if total_len <= self.capacity() {
            self.prepend_unchecked(&mut other);
            Ok(())
        }
        else {
            Err(errtxt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// prepend buffer with &str
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    ///
    /// let mut b: BFRDYN<256> = "coding".into();
    /// let some_str = "I love ";
    ///
    /// b.prepend_str(some_str);
    /// assert_eq!("I love coding", b.as_str());
    /// assert_eq!(13, b.len());
    /// ```
    ///
    pub fn prepend_str(&mut self, text: &str) -> Result<(), String> {
        let total_len = self.len + text.len();
        if total_len <= self.capacity() {
            let mut other: Self = text.into();
            self.prepend_unchecked(&mut other);
            Ok(())
        }
        else {
            Err(errtxt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// append current buffer with another buffer
    /// # example 
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut a: BFRDYN<125> = "I love".into();
    /// let b: BFRDYN<125> = " coding".into();
    ///
    /// a.append(b);
    /// assert_eq!("I love coding", a.to_string());
    /// assert_eq!(13, a.len());
    /// ```
    ///
    pub fn append(&mut self, other: Self) -> Result<(), String> {
        let total_len = self.len + other.len;
        if total_len <= self.capacity() {
            for i in 0..(other.len) {
                self.arr[self.len+i] = other.arr[i]
            }
            self.len += other.len;
            Ok(())
        }
        else {
            Err(errtxt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// append current buffer with &str
    /// # example 
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<125> = "I love".into();
    /// let some_str = " coding";
    ///
    /// b.append_str(some_str);
    /// assert_eq!("I love coding", b.to_string());
    /// assert_eq!(13, b.len());
    /// ```
    ///
    pub fn append_str(&mut self, text: &str) -> Result<(), String> {
        let total_len = self.len + text.len();
        if total_len <= self.capacity() {
            for i in 0..text.len() {
                self.arr[self.len+i] = text.as_bytes()[i]
            }
            self.len += text.len();
            Ok(())
        }
        else {
            Err(errtxt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// append current buffer with &ch
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<125> = "Happy condin".into();
    ///
    /// b.append_ch('g');
    /// assert_eq!("Happy conding", b.to_string());
    /// assert_eq!(13, b.len());
    /// ```
    ///
    pub fn append_ch(&mut self, c: char) -> Result<(), String> {
        let total_len = self.len + c.len_utf8();
        if total_len <= self.capacity() {
            for i in 0..c.len_utf8() {
                self.arr[self.len+i] = c as u8
            }
            self.len += c.len_utf8();
            Ok(())
        }
        else {
            Err(errtxt::not_enough_capacity(self.capacity(), total_len))
        }
    }

    /// shift value to right, leave original value
    /// this function will expand the buffer value len by 1
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "Amazing".into();
    /// b.rshift(6).unwrap();
    /// assert_eq!("Amazingg", b.as_str());
    /// assert_eq!(8, b.len());
    /// ```
    ///
    pub fn rshift(&mut self, pos: usize) -> Result<(), String> {
        if pos < self.len && self.len < self.capacity() {
            let mut idx = self.len;
            while idx > pos {
                self.arr[idx] = self.arr[idx-1];
                idx -= 1;
            }
            self.len += 1;
            Ok(())
        }
        else {
            Err(errtxt::not_valid_index(self.len, pos))
        }
    }
    
    /// shift value to left 
    /// this function will shrink the buffer len by 1
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "Amazing".into();
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
            Err(errtxt::not_valid_index(self.len, pos))
        }
    }

    /// take and remove a value from buffer
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "AmazZing".into();
    /// let x = b.take(4).unwrap();
    /// assert_eq!("Amazing", b.as_str());
    /// assert_eq!(7, b.len());
    /// assert_eq!('Z', x as char);
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
    /// use cbfr::core::cb::BFRDYN;
    /// let mut a: BFRDYN<256> = "Amng".into();
    /// let b: BFRDYN<256> = "azi".into();
    /// a.insert(b, 2).unwrap();
    /// assert_eq!("Amazing", a.as_str());
    /// assert_eq!(7, a.len());
    /// ```
    ///
    pub fn insert(&mut self, other: Self, pos: usize) -> Result<(), String> {
        let total_len = self.len + other.len;
        if total_len <= self.capacity() && pos < self.len {
            let mut idx = pos;
            for i in 0..other.len {
                self.rshift(idx)?;
                self.arr[idx] = other.arr[i];
                idx += 1;
            }
            Ok(())
        }
        else {
            Err(errtxt::not_enough_capacity(self.capacity(), total_len))
        }
    }
    
    /// insert with a char at a given position
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "Amaing".into();
    /// b.insert_ch('Z', 3).unwrap();
    /// assert_eq!("AmaZing", b.as_str());
    /// assert_eq!(7, b.len());
    /// ```
    ///
    pub fn insert_ch(&mut self, c: char, pos: usize) -> Result<(), String> {
        if self.len < self.capacity() {
            self.rshift(pos)?;
            self.arr[pos] = c as u8;
            Ok(())
        }
        else {
            Err(errtxt::not_enough_capacity(self.capacity(), self.len+1))
        }
    }

    /// reverse order of items in buffer
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "12345".into();
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
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "cgahb".into();
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
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "cgahb".into();
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

    /// trim space on left side
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "  L ove".into();
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

    /// trim space on right side
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "Lov e  ".into();
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
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = " Lov e  ".into();
    /// b.trim();
    /// assert_eq!("Lov e", b.as_str());
    /// assert_eq!(5, b.len());
    /// ```
    ///
    pub fn trim(&mut self) { self.ltrim(); self.rtrim(); }

    /// convert to lowercase
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "LoVE".into();
    /// b.lower();
    /// assert_eq!("love", b.as_str());
    /// assert_eq!(4, b.len());
    /// ```
    ///
    pub fn lower(&mut self) {
        for c in self.arr.iter_mut() {
            if *c <= 90 && *c >= 65 {
                *c = *c + 32;
            }
        }
    }

    /// convert to uppercase
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "loVe".into();
    /// b.upper();
    /// assert_eq!("LOVE", b.as_str());
    /// assert_eq!(4, b.len());
    /// ```
    ///
    pub fn upper(&mut self) {
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
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "lOVE".into();
    /// b.title();
    /// assert_eq!("Love", b.as_str());
    /// assert_eq!(4, b.len());
    /// ```
    ///
    pub fn title(&mut self) {
        self.lower();
        self.arr[0] -= 32;
    }
    
    /// to proper
    /// convert all value to lowercase but uppercase for every first letters
    /// # example
    /// ```
    /// use cbfr::core::cb::BFRDYN;
    /// let mut b: BFRDYN<256> = "damN i loVe iNdoNESia".into();
    ///
    /// b.proper();
    /// assert_eq!("Damn I Love Indonesia", b.as_str());
    /// assert_eq!(21, b.len());
    /// ```
    pub fn proper(&mut self) {
        let mut change_next = false;
        self.title();
        for (idx, c) in self.arr.iter_mut().enumerate() {
            if change_next && *c != ' ' as u8 {
                *c = *c - 32;
                change_next = false;
            }
            if idx < self.len-1 && *c == ' ' as u8 {
                change_next = true;
            }
        }
    }

}

