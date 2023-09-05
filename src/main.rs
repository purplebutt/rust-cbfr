use cbfr::prelude::BFRDYN;


fn main () {
    let mut b: BFRDYN = "Hello".into();
    println!("Before mutate: {b}");

    // let mr = unsafe { b.bytes_mut() };
    // mr[0] = 'Z' as u8;
    // println!("After mutate: {b}");
    // assert_eq!("Zello", b.as_ref());

    let mr = unsafe { b.bytes_mut() };
    mr[5] = b'!';
    unsafe { b.increase_len(1); }
    println!("After mutate: {b}");
    assert_eq!("Hello!", b.as_ref());

    let myb = BFRDYN::withcap::<64>();
    let l = myb.capacity();
    println!("{l}");
}

// quick start
#[allow(unused)]
fn main1() {
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

    for b in b1.as_bytes().iter() {
        if *b == b'o' { break }
        println!("{b}")
    }

    let myb: BFRDYN = "Test".into();
    let mut by = myb.as_bytes();
    by[0] = 'Z' as u8;

    println!("myb: {myb}");
    println!("by: {by:?}");
    println!("myb: {myb:?}");
}

