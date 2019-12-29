use std::fs;
use std::str::Lines;

fn calc_req_fuel(mass: i32) -> i32 {
    let fuel: i32 = mass / 3;
    let missed: i32 = fuel - 2;
    return missed;
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename: String = String::from("input.txt");
    let contents: String = fs::read_to_string(filename)?;
    let mut lines: Lines = contents.lines();
    let deneme: i32 = lines
        .map(|s| s.parse().unwrap())
        .map(calc_req_fuel)
        .fold(0, |acc: i32, curr: i32| acc + curr);
    println!("With text:\n{:?}", deneme);
    Ok(())
}

#[test]
fn another() {
    assert_eq!(calc_req_fuel(12), 2);
    assert_eq!(calc_req_fuel(14), 2);
    assert_eq!(calc_req_fuel(1969), 654);
    assert_eq!(calc_req_fuel(100756), 33583);
}
