use cbfr::CBfr;


fn main() {
    let mut bfr = [0; 256];
    let txt = "Abc";

    let mut cbfr: CBfr = (&mut bfr[..], txt).into();

    println!("{}", cbfr);
    println!("{}", cbfr.len());
    cbfr.insert_str(3, "def");
    println!("{}", cbfr);
    println!("{}", cbfr.len());
}