#[allow(overflowing_literals)]
fn main() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000 as u8, 232);

    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);
    
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);
    

    
    unsafe {
      
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
      
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
