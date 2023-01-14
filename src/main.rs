use cbfr::CBfr;


fn main() {
    let mut bfr = [0; 5];
    let txt = "Abcde";

    let mut cbfr: CBfr = (&mut bfr[..], txt).into();

    println!("{}", cbfr);
    println!("{}", cbfr.len());
    let x = cbfr.take(3, 2);
    println!("{}", x);
    println!("{}", cbfr);
}