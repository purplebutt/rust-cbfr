pub mod core;
//pub mod helper;

// use crate::core::b125::B125;
use crate::core::cb::BFRDYN;


fn main() {
    let mut b:BFRDYN = "this is just a test".into();

    let mut b2: BFRDYN<125> = BFRDYN::new();

    b2.append_str("text").unwrap(); 
    println!("{}", b2);

    b.reverse();

    let v = b.to_vec(' ');

    println!("{}", b);
    println!("{:?}", v);

    let mut c:BFRDYN = "cdba".into();
    c.sort();
    println!("{}", c);
}
