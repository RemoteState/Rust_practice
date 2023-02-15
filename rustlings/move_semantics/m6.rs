fn main() {
    let mut data = "Rust is great!".to_string();
    
    get_char(&mut data);
    
    string_uppercase(data);
}

fn get_char(x: &mut String) -> char {
    x.chars().last().unwrap()
}

fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    
    println!("{}", data);
}