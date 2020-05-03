fn main() {
    let x = 1;
    let y : Option<u8> = None;

    if let Some(i) = y {
        println!("{}", x + i);
    }
    else {
        println!("None");
    }
}
