pub mod core;
pub mod helper;


// use crate::core::b125::B125;
use crate::core::cb::BFRDYN;

fn main() {
    // let b: B125 = "Nice".into();
    // let c: B125 = "Nice".into();


   // if b == c {
   //     println!("Its ==")
   // }
   // else{
   //     println!("Not ==")
   // }

    //let x = b.checksum() + c.checksum();
    //println!("b.min(c) : {}", b.min(c));
    //println!("b+c: {}", b + c);

    //for i in b.iter() {
    //    println!("-> {}", *i as char)
    //}


    let mut y: BFRDYN = "Hello".into();
    y.append_ch('a').unwrap();
    y.append_ch('b').unwrap();
    y.append_ch('c').unwrap();
    y.append_ch('A').unwrap();
    
    let mut x: BFRDYN<512> = "World".into();
    x.append_ch('3').unwrap();
    x.append_ch('Z').unwrap();
    x.append_ch('M').unwrap();
    x.append_ch('u').unwrap();
    
    println!("x: {}\ty: {}", y, x);
    y.upper();
    x.reverse();
    println!("x: {}\ty: {}", y, x);
}

