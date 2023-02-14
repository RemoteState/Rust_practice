#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    
    fn from(x: i32) -> Self {
        Number{value: x}
    }
    
}

fn main() {
    let num: Number = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = Number{value: 30};
    assert_eq!(num.value, 30);

    println!("Success!");
}
