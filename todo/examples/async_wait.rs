use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn boil_water() -> String {
    println!("お湯: 沸かし始める");
    sleep(Duration::from_secs(1)).await;
    println!("お湯: 沸いた");
    "お湯".to_string()
}

async fn toast_bread() -> String {
    println!("パン: 焼き始める");
    sleep(Duration::from_secs(1)).await;
    println!("パン: 焼けた");
    "トースト".to_string()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let water = boil_water().await;
    let bread = toast_bread().await;
    let seconds = start.elapsed().as_secs_f64();
    println!(
        "順に待つ: {water}と{bread}、{seconds:.1} 秒"
    );

    let start = Instant::now();
    let (water, bread) =
        tokio::join!(boil_water(), toast_bread());
    let seconds = start.elapsed().as_secs_f64();
    println!(
        "同時に待つ: {water}と{bread}、{seconds:.1} 秒"
    );
}
