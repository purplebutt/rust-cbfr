pub mod helper;

use std::cell::UnsafeCell;
use std::fmt::Display;
use std::ptr::NonNull;
use helper::validate_cap;
use helper::validate_len;


/// Buffer to work with array of char or string
/// # Example:
/// ```
/// use cbfr::CBfr;
/// 
/// let mut buffer = [0; 512];
/// let mut cbfr = CBfr::new(&mut buffer);
/// cbfr.append_ch('H');
/// cbfr.append_ch('e');
/// cbfr.append_ch('l');
/// cbfr.append_ch('l');
/// cbfr.append_ch('o');
/// 
/// assert_eq!("Hello", cbfr.to_string());
/// ```
pub struct CBfr<'a> {
    bfr: &'a mut [u8],
    len: usize
}

impl<'a> Display for CBfr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let partial = &self.bfr[0..self.len];
        let txt;
        unsafe {
            txt = std::str::from_utf8_unchecked(partial)
        }
        write!(f, "{}", txt)
    }
}

impl<'a> From<(&'a mut [u8], &str)> for CBfr<'a> {
    /// Create buffer from (&mut[u8], &str)
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let text = "Hello";
    /// let mut bfr:CBfr = (&mut buffer[..], text).into();
    /// 
    /// assert_eq!("Hello", bfr.to_string())
    /// ```
    /// # Panic:
    /// Panic if (current buffer len + str len) > buffer capacity.
    /// # Panic Example:
    /// ```
    /// let mut buffer = [0; 5];    // create buffer with capacity of 5
    /// let text = "123456";     // text length is 6
    /// //let bfr:CBfr = (&mut buffer[..], text).into()   // will panic
    /// ```
    fn from(value: (&'a mut [u8], &str)) -> Self {
        match validate_cap(value.0.len(), 0, value.1.len()) {
            Ok(()) => {
                let s = Self { bfr: value.0, len: value.1.len() };
                for (i, c) in value.1.bytes().enumerate() {
                    s.bfr[i] = c
                }
                return s;
            }
            Err(e) => panic!("{}", e)
        }
    }
}

impl<'a> CBfr<'a> {
    /// Create a new buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut cbfr = CBfr::new(&mut buffer);
    /// 
    /// cbfr.append_ch('A');
    /// cbfr.append_ch('z');
    /// 
    /// assert_eq!("Az", cbfr.to_string());
    /// assert_eq!(2, cbfr.len());
    /// ```
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { bfr: buffer, len: 0 }
    }

    /// Create a clone
    /// This function did not implement Clone trait
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 256];
    /// let mut b1 = CBfr::new(&mut buffer1);
    /// b1.append_str("Hello World");
    ///
    /// let mut buffer2 = buffer1.clone();
    /// let mut b2 = CBfr::clone(&mut buffer2);
    /// b2.append_ch('!');
    /// assert_eq!("Hello World!", b2.to_string());
    /// 
    pub fn clone(buffer: &'a mut [u8]) -> Self {
        let mut len = 0usize;
        for i in buffer.iter() {
            if *i != 0 {
                len += 1
            }
        }
        Self { bfr: buffer, len }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(&self.bfr)
        }
    }

    /// Return the len of buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut bfr = CBfr::new(&mut buffer);
    /// 
    /// bfr.append_ch('A');
    /// bfr.append_ch('b');
    /// bfr.append_ch('c');
    /// 
    /// assert_eq!(3, bfr.len());
    /// ```
    /// 
    pub fn len(&self) -> usize { self.len }

    /// Return true if buffer is full otherwise return false
    pub fn is_full(&self) -> bool { self.len == self.bfr.len() }

    /// Return available space
    pub fn available(&self) -> usize { self.bfr.len() - self.len }

    /// Return the capacity of buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut bfr = CBfr::new(&mut buffer);
    /// 
    /// assert_eq!(256, bfr.cap());
    /// ```
    /// 
    pub fn cap(&self) -> usize { self.bfr.len() }

    /// Clear buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut bfr = CBfr::new(&mut buffer);
    /// 
    /// bfr.append_ch('H');
    /// bfr.append_ch('e');
    /// bfr.append_ch('l');
    /// bfr.append_ch('l');
    /// bfr.append_ch('o');
    /// 
    /// assert_eq!("Hello", bfr.to_string());
    /// assert_eq!(5, bfr.len());
    /// 
    /// bfr.clear();
    /// assert_eq!("", bfr.to_string());
    /// assert_eq!(0, bfr.len());
    /// ```
    /// 
    pub fn clear(&mut self) {
        for c in self.bfr.iter_mut() {
            if *c != 0 { *c = 0 }
        }
        self.len = 0;
    }

    /// Append a char into buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut cbfr = CBfr::new(&mut buffer);
    /// 
    /// cbfr.append_ch('A');
    /// cbfr.append_ch('b');
    /// cbfr.append_ch('c');
    /// 
    /// assert_eq!("Abc", cbfr.to_string());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (current buffer len + 1) > buffer capacity.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 3];    // buffer with capacity only 3
    /// let mut bag = CBfr::new(&mut buffer);
    /// 
    /// bag.append_ch('A');
    /// bag.append_ch('b');
    /// bag.append_ch('c');
    /// assert_eq!(3, bag.len());  // len is now 3, bag is now full
    /// //bag.append_ch('d');       // will panic, since bag is full already
    /// ```
    pub fn append_ch(&mut self, c: char) {
        match validate_cap(self.bfr.len(), self.len, 1) {
            Ok(()) => {
                self.bfr[self.len] = c as u8;
                self.len += 1;
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Append other buffer with current buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 256];
    /// let mut buffer2 = [0; 256];
    /// 
    /// let mut b1 = CBfr::new(&mut buffer1);
    /// let mut b2 = CBfr::new(&mut buffer2);
    /// 
    /// b1.append_str("Hello");
    /// b2.append_str("World");
    /// 
    /// b1.append_ch(' ');  // add space
    /// b1.append(b2);
    /// 
    /// assert_eq!("Hello World", b1.to_string());
    /// assert_eq!(11, b1.len());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (current buffer.len() + other buffer.len()) > current buffer capacity.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 5];    // buffer with capacity only 3
    /// let mut buffer2 = [0; 5];    // buffer with capacity only 3
    /// 
    /// let mut text1 = CBfr::new(&mut buffer1);
    /// let mut text2 = CBfr::new(&mut buffer2);
    /// 
    /// text1.append_str("Hello");
    /// text2.append_str("World");
    /// 
    /// //text1.append(text2);    // will panic!
    /// ```
    pub fn append(&mut self, other: Self) {
        match validate_cap(self.bfr.len(), self.len, other.len) {
            Ok(()) => {
                for i in 0..other.len {
                    self.bfr[self.len+i] = other.bfr[i]
                }
                self.len += other.len;
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Append a &str into buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut cbfr = CBfr::new(&mut buffer);
    /// 
    /// cbfr.append_str("Hello World");
    /// 
    /// assert_eq!("Hello World", cbfr.to_string());
    /// assert_eq!(11, cbfr.len());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (current buffer.len() + &str.len()) > buffer capacity.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 3];    // buffer with capacity only 3
    /// let mut bag = CBfr::new(&mut buffer);
    /// 
    /// //bag.append_str("Nice");     // "Nice".len() is 4, but bag capacity only 3. Oopps!
    /// ```
    pub fn append_str(&mut self, text: &str) {
        match validate_cap(self.bfr.len(), self.len, text.len()) {
            Ok(()) => {
                for (i, c) in text.bytes().enumerate() {
                    self.bfr[self.len+i] = c
                }
                self.len += text.len();
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Shift data in buffer to the right
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut cbfr = CBfr::new(&mut buffer);
    /// 
    /// cbfr.append_str("Amazing");
    /// cbfr.rshift(2, 3, '#');
    /// 
    /// assert_eq!("Am###azing", cbfr.to_string());
    /// assert_eq!(10, cbfr.len());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (current buffer.len() + step) > buffer capacity.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 8];    // buffer with capacity only 5
    /// let mut bag = CBfr::new(&mut buffer);
    /// 
    /// bag.append_str("Wow");
    /// bag.rshift(1, 2, '#');
    /// assert_eq!("W##ow", bag.to_string());
    /// assert_eq!(5, bag.len());
    /// 
    /// bag.rshift(0, 3, '@');
    /// assert_eq!("@@@W##ow", bag.to_string());
    /// assert_eq!(8, bag.len());           // len is now 8
    /// assert_eq!(true, bag.is_full());     // bag is now full
    /// 
    /// // bag.rshift(1, 1, '#')       // can not add more, because bag is full. Will panic
    /// ```
    pub fn rshift(&mut self, from: usize, step: usize, fill_with: char) {
        match validate_cap(self.bfr.len(), self.len, step) {
            Ok(()) => {
                let from = from+1;
                for _ in 0..step {
                    self.len += 1;
                    let mut i = self.len;
                    while i > from {
                        i -= 1;
                        self.bfr[i] = self.bfr[i-1];
                        self.bfr[i-1] = fill_with as u8;
                    }
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Shift data in buffer to the left
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut cbfr = CBfr::new(&mut buffer);
    /// 
    /// cbfr.append_str("Amazing");
    /// cbfr.lshift(2, 3);      // remove 3 char start from third item (zero-based index)
    /// 
    /// assert_eq!("Amng", cbfr.to_string());
    /// assert_eq!(4, cbfr.len());
    /// ```
    pub fn lshift(&mut self, from: usize, step: usize) {
        for _ in 0..step {
            for i in from..self.len {
                if i < self.len-1 {
                    self.bfr[i] = self.bfr[i+1]
                }
                else {
                    self.bfr[i] = 0
                }
            }
            self.len -= 1;
        }
    }

    /// Prepend a char
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut cbfr = CBfr::new(&mut buffer);
    /// 
    /// cbfr.append_str("Amazing");
    /// cbfr.prepend_ch('M');
    /// 
    /// assert_eq!("MAmazing", cbfr.to_string());
    /// assert_eq!(8, cbfr.len());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (current buffer.len() + 1) > buffer capacity.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 3];    // buffer with capacity only 3
    /// let mut bag = CBfr::new(&mut buffer);
    /// 
    /// bag.append_str("Wow");
    /// //bag.prepend_ch("W");     // will panic
    /// ```
    pub fn prepend_ch(&mut self, c: char) {
        self.rshift(0, 1, '#');
        self.bfr[0] = c as u8;
    }
    
    /// Prepend a &str
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut cbfr = CBfr::new(&mut buffer);
    /// 
    /// cbfr.append_str("World");
    /// cbfr.prepend_ch(' ');
    /// cbfr.prepend_str("Hello");
    /// 
    /// assert_eq!("Hello World", cbfr.to_string());
    /// assert_eq!(11, cbfr.len());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (current buffer.len() + &str.len()) > buffer capacity.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 5];    // buffer with capacity only 5
    /// let mut bag = CBfr::new(&mut buffer);
    /// 
    /// bag.append_str("ow");            
    /// assert_eq!(3, bag.available()); // only 3 space available
    /// //bag.prepend_str("Hell");        // will panic. "Hell".len() is 4 but only 3 space available
    /// //assert_eq!("Hellow", bag.to_string());
    /// //assert_eq!(5, bag.len());
    /// ```
    pub fn prepend_str(&mut self, text: &str) {
        self.rshift(0, text.len(), '#');
        for (i, c) in text.bytes().enumerate() {
            self.bfr[i] = c
        }
    }

    /// Prepend other buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 256];
    /// let mut buffer2 = [0; 256];
    /// 
    /// let mut b1 = CBfr::new(&mut buffer1);
    /// let mut b2 = CBfr::new(&mut buffer2);
    /// 
    /// b1.append_str("World");
    /// b2.append_str("Hello ");
    /// 
    /// b1.prepend(b2);
    /// 
    /// assert_eq!("Hello World", b1.to_string());
    /// assert_eq!(11, b1.len());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (current buffer.len() + other.len()) > buffer capacity.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 5];    // buffer with capacity only 5
    /// let mut buffer2 = [0; 5];    // buffer with capacity only 5
    /// 
    /// let mut bag1 = CBfr::new(&mut buffer1);
    /// let mut bag2 = CBfr::new(&mut buffer2);
    /// 
    /// bag1.append_str("ow");            
    /// bag2.append_str("Hell");            
    /// 
    /// assert_eq!(3, bag1.available()); // only 3 space available
    /// //bag1.prepend(bag2);        // will panic. "Hell".len() is 4 but only 3 space available
    /// //assert_eq!("Helow", bag1.to_string());
    /// //assert_eq!(5, bag1.len());
    /// ```
    pub fn prepend(&mut self, other: Self) {
        self.rshift(0, other.len(), '#');
        for i in 0..other.len {
            self.bfr[i] = other.bfr[i]
        }
    }

    /// Take some items form buffer
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 20];
    /// 
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("I love you so much");
    /// assert_eq!(18, b.len());     // b.len() is 18
    /// 
    /// let t1 = b.take(7, 11);     // get item from 7 to the last item
    /// assert_eq!("you so much", t1);
    /// assert_eq!("I love ", b.to_string());
    /// assert_eq!(7, b.len());
    /// 
    /// ```
    /// # Panic:
    /// Panic if (at + how_many) > buffer length.
    /// # Panic Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 20];
    /// 
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("I love you so much");
    /// assert_eq!(18, b.len());     // b.len() is 18
    /// 
    /// // let t1 = b.take(7, 12);     // will panic. 7+12=19, but length is 18.
    /// let t1 = b.take(6, 12);     // this one is ok
    /// assert_eq!(" you so much", t1);
    /// assert_eq!("I love", b.to_string());
    /// assert_eq!(6, b.len());
    /// 
    pub fn take(&mut self, at: usize, how_many: usize) -> String {
        match validate_len(self.len, at+how_many-1) {
            Ok(()) => {
                match validate_len(self.len, at) {
                    Ok(()) => {
                        let mut s = String::new();
                        for i in at..at+how_many {
                            s.push(self.bfr[i] as char)
                        }
                        self.lshift(at, how_many);
                        return s;
                    }
                    Err(e) => panic!("{}", e)
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Insert chart at position
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 20];
    /// let mut b = CBfr::new(&mut buffer);
    /// b.append_str("Ble");
    /// b.insert_ch(2, 'u');
    /// assert_eq!(4, b.len());
    /// assert_eq!("Blue", b.to_string());
    /// ```
    /// # Panic
    /// Panic if:
    ///     1. Current buffer is full already, or
    ///     2. at > length 
    /// 
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 3];    // buffer capacity is 3
    /// let mut b = CBfr::new(&mut buffer1);
    /// b.append_str("Ble");
    /// assert_eq!(true, b.is_full());  // buffer is full
    /// // b.insert_ch(2, 'u');            // can not insert, buffer is full
    /// 
    /// let mut buffer2 = [0; 256];
    /// let mut c = CBfr::new(&mut buffer2);
    /// c.append_str("Amazing");
    /// assert_eq!(7, c.len());  // buffer len is 7
    /// // c.insert_ch(7, 'u');  // trying to insert on index 7 when len is 7. Will panic
    /// ```
    pub fn insert_ch(&mut self, at: usize, c: char) {
        match validate_cap(self.bfr.len(), self.len, 1) {
            Ok(()) => {
                match validate_len(self.len, at) {
                    Ok(()) => {
                        self.rshift(at, 1, '#');
                        self.bfr[at] = c as u8;
                    },
                    Err(e) => panic!("{}", e)
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Insert &str at position
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 20];
    /// let mut b = CBfr::new(&mut buffer);
    /// b.append_str("I and you");
    /// b.take(2, 3);
    /// assert_eq!("I  you", b.to_string());
    /// assert_eq!(6, b.len());
    /// 
    /// b.insert_str(2, "love");
    /// assert_eq!("I love you", b.to_string());
    /// assert_eq!(10, b.len());
    /// 
    /// ```
    /// # Panic
    /// Panic if:
    ///     1. Current buffer is full already, or
    ///     2. at > length 
    /// 
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 7];    // buffer capacity is 7
    /// let mut b = CBfr::new(&mut buffer1);
    /// b.append_str("Amazing");
    /// assert_eq!(true, b.is_full());  // buffer is full
    /// // b.insert_str(0, "The ");            // can not insert, buffer is full
    /// 
    /// let mut buffer2 = [0; 256];
    /// let mut c = CBfr::new(&mut buffer2);
    /// c.append_str("abc");
    /// // c.insert_str(4, "def");  // will panic
    /// c.insert_str(3, "def");
    /// assert_eq!(6, c.len());
    /// assert_eq!("abcdef", c.to_string());
    /// ```
    pub fn insert_str(&mut self, at: usize, text: &str) {
        match validate_cap(self.bfr.len(), self.len, text.len()) {
            Ok(()) => {
                match validate_len(self.len+1, at) {
                    Ok(()) => {
                        for (i, c) in text.bytes().enumerate() {
                            self.rshift(at+i, 1, '#');
                            self.bfr[at+i] = c;
                        }
                    },
                    Err(e) => panic!("{}", e)
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Insert &str at position
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 256];
    /// let mut b1 = CBfr::new(&mut buffer1);
    /// 
    /// let mut buffer2 = [0; 20];
    /// let mut b2 = CBfr::new(&mut buffer2);
    /// 
    /// b1.append_str("I  you");
    /// b2.append_str("love");
    /// 
    /// b1.insert(2, b2);
    /// assert_eq!("I love you", b1.to_string());
    /// assert_eq!(10, b1.len());
    /// 
    /// ```
    /// # Panic
    /// Panic if:
    ///     1. Current buffer is full already, or
    ///     2. at > length 
    /// 
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 8];    // buffer capacity is 7
    /// let mut b1 = CBfr::new(&mut buffer1);
    /// 
    /// let mut buffer2 = [0; 256];
    /// let mut b2 = CBfr::new(&mut buffer2);
    /// 
    /// b1.append_str("Amazing ");
    /// assert_eq!(true, b1.is_full());     // b1 is now full
    /// b2.append_str("Spiderman");
    /// //b1.append(b2);    // can not add more. Will panic
    /// //assert_eq!("Amazing Spiderman", b1.to_string());
    /// //assert_eq!(17, b1.len());
    /// ```
    /// 
    /// # Other Panic Example
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer1 = [0; 256];    // buffer capacity is 7
    /// let mut b1 = CBfr::new(&mut buffer1);
    /// 
    /// let mut buffer2 = [0; 256];
    /// let mut b2 = CBfr::new(&mut buffer2);
    /// 
    /// b1.append_str("Amazing ");
    /// b2.append_str("Spiderman");
    /// assert_eq!(8, b1.len());    // len is 8
    /// //b1.insert(9, b2);         // will panic. Len is 8 but try to insert at 9
    /// //assert_eq!("Amazing Spiderman", b1.to_string());
    /// //assert_eq!(17, b1.len());
    /// ```
    pub fn insert(&mut self, at: usize, other: Self) {
        match validate_cap(self.bfr.len(), self.len, other.len()) {
            Ok(()) => {
                match validate_len(self.len+1, at) {
                    Ok(()) => {
                        for i in 0..other.len {
                            self.rshift(at+i, 1, '#');
                            self.bfr[at+i] = other.bfr[i]
                        }
                    },
                    Err(e) => panic!("{}", e)
                }
            }
            Err(e) => panic!("{}", e)
        }
    }

    /// Reverse item
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("Abcdef");
    /// b.reverse();
    /// 
    /// assert_eq!("fedcbA", b.to_string());
    /// ```
    pub fn reverse(&mut self) {
        let mid = self.len/2;
        let mut idx = (0usize, self.len-1);
        while idx.0 < mid {
            let temp = self.bfr[idx.0];
            self.bfr[idx.0] = self.bfr[idx.1];
            self.bfr[idx.1] = temp;
            idx.0 += 1;
            idx.1 -= 1;
        }
    }

    /// Sort item ascending using linear search algorithm
    /// This operation takes O(n) time complexity
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("efAdcb");
    /// b.sort();
    /// 
    /// assert_eq!("Abcdef", b.to_string());
    /// ```
    pub fn sort(&mut self) {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 0..(self.len-1) {
                if self.bfr[i+1] < self.bfr[i] {
                    let temp = self.bfr[i];
                    self.bfr[i] = self.bfr[i+1];
                    self.bfr[i+1] = temp;
                    sorted = false;
                }
            }
        }
    }

    /// Sort item descending using linear search algorithm
    /// This operation takes O(n) time complexity
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("efAdcb");
    /// b.sort_desc();
    /// 
    /// assert_eq!("fedcbA", b.to_string());
    /// ```
    pub fn sort_desc(&mut self) {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 0..(self.len-1) {
                if self.bfr[i+1] > self.bfr[i] {
                    let temp = self.bfr[i];
                    self.bfr[i] = self.bfr[i+1];
                    self.bfr[i+1] = temp;
                    sorted = false;
                }
            }
        }
    }

    /// Remove space on the left
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("  Hello");
    /// b.ltrim();
    /// 
    /// assert_eq!("Hello", b.to_string());
    /// assert_eq!(5, b.len());
    /// ```
    pub fn ltrim(&mut self) {
        if self.len > 1 {
            let mut c = self.bfr[0];
            let mut idx = self.len;
            while c == 32 && idx > 1 {
                self.lshift(0, 1);
                c = self.bfr[0];
                idx -= 1;
            }
        }
    }

    /// Remove space on the right
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("Hello  ");
    /// b.rtrim();
    /// 
    /// assert_eq!("Hello", b.to_string());
    /// assert_eq!(5, b.len());
    /// ```
    pub fn rtrim(&mut self) {
        if self.len > 1 {
            let mut last = self.bfr[self.len-1];
            while last == 32 {
                self.bfr[self.len-1] = 0;
                self.len -= 1;
                last = self.bfr[self.len-1];
            }
        }
    }

    /// Remove space on the right and left
    /// This function is actually calling self.ltrim() and self.rtrim()
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str("  Hello  ");
    /// b.trim();
    /// 
    /// assert_eq!("Hello", b.to_string());
    /// assert_eq!(5, b.len());
    /// ```
    pub fn trim(&mut self) {
        self.ltrim();
        self.rtrim();
    }

    /// Trim all spaces
    /// Takes O(n) time complexity
    /// # Example:
    /// ```
    /// use cbfr::CBfr;
    /// let mut buffer = [0; 256];
    /// let mut b = CBfr::new(&mut buffer);
    /// 
    /// b.append_str(" Hello   World ");
    /// b.trimall();
    /// 
    /// assert_eq!("Hello World", b.to_string());
    /// assert_eq!(11, b.len());
    /// ```
    pub fn trimall(&mut self) {
        self.ltrim();
        self.rtrim();

        if self.len > 1 {
            let mut last = self.bfr[0];
            for i in 1..self.len {
                if last == 32 {
                    if self.bfr[i-1] == 32 {
                        self.lshift(i-1, 1)
                    }
                }
                if i+1 == self.len { break; }
                last = self.bfr[i]
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter { arr: &self.bfr[0..self.len], idx: 0 }
    }

    // todo
    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut { arr: &mut self.bfr[0..self.len], idx: 0 }
    }

    pub fn bytes(&self) -> Bytes {
        Bytes { arr: &self.bfr[0..self.len], idx: 0 }
    }

    fn find(&self, target: &str) -> Option<usize> {
        todo!()
    }

    fn replace(&mut self, target: &str, with: &str) {
        todo!()
    }

    fn left(&self, how_many: usize) -> &str {
        todo!()
    }

    fn right(&self, how_many: usize) -> &str {
        todo!()
    }

    fn mid(&self, at: usize, how_many: usize) -> &str {
        todo!()
    }
}


// Iterators
pub struct Iter<'a> {
    arr: &'a [u8],
    idx: usize
}
impl<'a> Iterator for Iter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.arr.len() {
            let item = Some(self.arr[self.idx] as char);
            self.idx += 1;
            return item
        }
        None
    }
}

#[allow(dead_code)]
pub struct IterMut<'a> {
    arr: &'a mut [u8],
    idx: usize
}
impl<'a> Iterator for IterMut<'a> {
    type Item = *mut u8;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    } 
}


pub struct Bytes<'a> {
    arr: &'a [u8],
    idx: usize
}
impl<'a> Iterator for Bytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.arr.len() {
            let item = Some(self.arr[self.idx]);
            self.idx += 1;
            return item
        }
        None
    }
}