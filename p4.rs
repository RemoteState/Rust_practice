
// FIX the error and IMPLEMENT the code
fn main() {
    let mut v = Vec::from([1, 2, 3, 4, 5]);
    for i in 0..5 {
        println!("{:?}", v[i])
    }

    for i in 0..5 {
       // IMPLEMENT the code here...
       v[i] += 1 
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
