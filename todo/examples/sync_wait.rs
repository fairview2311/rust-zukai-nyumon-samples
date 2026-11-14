use std::thread::sleep;
use std::time::{Duration, Instant};

fn boil_water() -> String {
    println!("お湯: 沸かし始める");
    sleep(Duration::from_secs(1));
    println!("お湯: 沸いた");
    "お湯".to_string()
}

fn toast_bread() -> String {
    println!("パン: 焼き始める");
    sleep(Duration::from_secs(1));
    println!("パン: 焼けた");
    "トースト".to_string()
}

fn main() {
    let start = Instant::now();
    let water = boil_water();
    let bread = toast_bread();
    let seconds = start.elapsed().as_secs_f64();
    println!(
        "{water}と{bread}ができた: {seconds:.1} 秒"
    );
}
