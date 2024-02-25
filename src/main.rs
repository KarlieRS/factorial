// fn factorial(n: i32) -> i32 {
//     match n {
//         0..=1 => n,
//         _ => factorial(n-1)*n
//     }
// }

fn main() {
    let n = 10;
    let f = factorial(n);
    println!("Factroial of {}: {}", n, f);
}
