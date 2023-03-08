pub mod core;
pub mod helper;


use crate::core::b125::B125;

fn main() {
    let mut b: B125 = "Nice".into();
    let mut c: B125 = "Nice".into();


    if b == c {
        println!("Its ==")
    }
    else{
        println!("Not ==")
    }

    let x = b.checksum() + c.checksum();
    //println!("b.min(c) : {}", b.min(c));
    //println!("b+c: {}", b + c);

    for i in b.iter() {
        println!("-> {}", *i as char)
    }

}
