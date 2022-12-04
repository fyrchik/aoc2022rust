use took::Timer;

fn main() {
    let timer = Timer::new();
    runner::jobs().iter().for_each(|j| {
        println!("{}: {}", j.1, j.0(j.2));
    });
    timer.took().describe("everything");
}
