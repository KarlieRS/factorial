use factorial::factorial;

/// main function testing factorial function
fn main() {
    println!("### Testing factorial function ###");
    println!("{}! = {:.2}", 5, factorial(5));
    println!("{}! = {:.2E}", 20, factorial(20));
    println!("Function returns i64 is used because max(i32) = {:E} and max(i64) = {:E}", i32::MAX, i64::MAX)
}
