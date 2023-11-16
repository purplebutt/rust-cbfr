use cbfr::prelude::BFRDYN;


#[allow(dead_code)]
fn demo1() {
    let mut b: BFRDYN = "Hello".into();
    println!("Before mutate: {b}");

    let containll = b.contain_str("el"); println!("Contain el: {}", containll);
    let containll = b.contain_str("l"); println!("Contain l: {}", containll);
    let containll = b.contain_str("Hel"); println!("Contain Hel: {}", containll);
    let containll = b.contain_str("ll"); println!("Contain ll: {}", containll);
    let containll = b.contain_str("lo"); println!("Contain lo: {}", containll);
    let containll = b.contain_str("Amazzing"); println!("Contain amazzing: {}", containll);

    // let mr = unsafe { b.bytes_mut() };
    // mr[0] = 'Z' as u8;
    // println!("After mutate: {b}");
    // assert_eq!("Zello", b.as_ref());

    let mr = unsafe { b.bytes_mut() };
    mr[5] = b'!';
    unsafe { b.increase_len(1); }
    println!("After mutate: {b}");
    assert_eq!("Hello!", <BFRDYN as AsRef<str>>::as_ref(&b));

    let myb = BFRDYN::withcap::<64>();
    let l = myb.capacity();
    println!("{l}");
}

#[allow(dead_code)]
fn demo2() {
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


use std::fs::write;

fn _write_with_cbfr() {
    let sometext = "Hello world!";
    let buffer: BFRDYN = sometext.into();
    let path = std::path::Path::new("./sample.txt");

    write(path, buffer).ok();
}

const CAP: usize = 8500;

fn sort_demo() -> u128 {
    let mut data = vec![];
    for _ in 0..CAP {
        let buf: BFRDYN<32> = "ziA4xaij7M23sjK08u1)".into();
        data.push(buf);
    }
    let d = std::time::Instant::now();
    for i in data.iter_mut() { i.sort(); }
    let etime = d.elapsed().as_millis();
    println!("exec time[sort]: {etime} milliseconds");
    let last = data.pop().unwrap();
    println!("result: {}", last.as_str());
    etime
}

fn isort_demo() -> u128 {
    let mut data = vec![];
    for _ in 0..CAP {
        let buf: BFRDYN<32> = "ziA4xaij7M23sjK08u1)".into();
        data.push(buf);
    }
    let d = std::time::Instant::now();
    for i in data.iter_mut() { i.isort(); }
    let etime = d.elapsed().as_millis();
    println!("exec time[isort]: {etime} milliseconds");
    let last = data.pop().unwrap();
    println!("result: {}", last.as_str());
    etime
}

fn main() {
   //  let slice: &[u8] = "Admin123".as_bytes();
   // 
   //  let mut buf = BFRDYN::<256>::from_slice(slice);
   //  buf.auto_len();
   //  println!("{}", buf.len());
   //  println!("{}", buf.as_str());
   //  return;
   //  
   //  demo1();
   //  demo2();
   //  write_with_cbfr();

    let sorttime = sort_demo();
    let isorttime = isort_demo();
    let dif = sorttime - isorttime;
    println!("time diff: {} milliseconds", dif);
}

