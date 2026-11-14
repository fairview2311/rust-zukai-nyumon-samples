use std::time::Instant;

fn is_prime(n: u32) -> bool {
    (2..)
        .take_while(|d| d * d <= n)
        .all(|d| !n.is_multiple_of(d))
}

fn main() {
    let start = Instant::now();
    let count =
        (2..2_000_000).filter(|&n| is_prime(n)).count();
    let seconds = start.elapsed().as_secs_f64();
    println!("素数の数: {count}（{seconds:.2} 秒）");
}
