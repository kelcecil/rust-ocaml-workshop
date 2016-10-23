fn add_n(n: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |x: i32| n + x)
}

#[test]
fn test_add_n() {
    let add_ten = add_n(10);
    println!("The answer to {} + 10 is: {}", 4, add_ten(4))
}