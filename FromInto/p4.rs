fn main() {
    let n: i16 = 256;

    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("problem in converting {:?}", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!");
}