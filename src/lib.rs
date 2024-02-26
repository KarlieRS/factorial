/// factorial(i8)->i64: returns factorial of a given number
pub fn factorial(n: i8) -> i64 {
    assert!(n > 0, "Argument can't be smaller than 0");
    assert!(n < 21, "Argument can't be bigger than 20 (overflow)");
    match n {
        0..=1 => 1i64,
        _ => factorial(n-1)*(n as  i64)
    }
}