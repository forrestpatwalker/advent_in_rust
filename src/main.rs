mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _day_1 = "day1".to_string();
    match args.get(1).as_deref() {
        Some(_day1) => day1::run(),
        _ => println!("Specify a day to run e.g. 'day1'")
    }
}
