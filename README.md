# cbfr
**A fast simple buffer running on stack, built on top of rust primitive array

Our links: 
- [Github] (https://github.com/purplebutt/rust-cbfr) 

What's new in version 0.1.5
- BFRDYN now implement Hash, you can now use it as key for HashMap (or HashSet) 
- new function "contain_str", "split", "split_incl", "split_incl_left"
- new function "split2", "split2_incl", "split2_incl_left"
- new function "starts_with", "ends_with", "pop", "popn", "take_head"

"as_str" is replaceable by "as_ref()" but in some situation, "as_str" is
very handy and can simplify our code, so we bring it back, it's no longer deprecated.

What's new in version 0.1.4..?
- BFRDYN now implement AsRef<[u8]>
- BFRDYN now implement std::borrow::Borrow<[u8]>
- BFRDYN now implement std::borrow::BorrowMut<[u8]>
- BFRDYN now implement std::ops::Deref

What's new in version 0.1.3..?
- Documentation fixed
- New function auto_len(), increase_len(), decrease_len()

What's new in version 0.1.2..?
- Code have been restructured, now it's much simpler and easy to import
- Helper functions are now excluded from prelude
- Some method and function are now const

## Quick start:

```rust

use cbfr::prelude::BFRDYN;


fn main() {
    let mut b1:BFRDYN = "I love ..".into();

    b1.reverse();
    let v = b1.to_vec(' ');

    println!("{}", b1);
    println!("{:?}", v);

    let mut b2: BFRDYN<512> = BFRDYN::new();
    b2.append_str("coding!").unwrap(); 
    println!("{}", b2);

    let mut b3:BFRDYN = "cbr".into();
    let b4:BFRDYN = "eda".into();
    b3.append(b4).unwrap();
    
    b3.sort(); 
    println!("{}", b3);
}

```

## Feedback:
If you have any suggestions, issues, feedback, or anything..?
[Send issues or feedback](https://github.com/purplebutt/rust-cbfr/issues)
