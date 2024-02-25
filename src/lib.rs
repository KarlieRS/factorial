pub fn factorial(n: i32) -> i32 {
    match n {
        0..=1 => 1i32,
        _ => factorial(n-1)*n
    }
}