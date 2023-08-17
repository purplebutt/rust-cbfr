# rust-cbfr
**A fast simple buffer running on stack, built on top of rust primitive array

Our links:
    - [Github] (https://github.com/purplebutt/rust-cbfr)
    - [Documentation] (https://docs.rs/cbfr/0.1.1/cbfr/core/cb/struct.BFRDYN.html)


## Quick start:

```
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
