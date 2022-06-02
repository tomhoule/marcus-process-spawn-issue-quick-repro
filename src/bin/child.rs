fn main() {
    let start = std::time::Instant::now();

    println!("counting time now");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!(
            "{} seconds elapsed",
            std::time::Instant::now().duration_since(start).as_secs()
        );
    }
}
