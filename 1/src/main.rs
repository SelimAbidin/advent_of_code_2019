use std::fs;
use std::str::Lines;

fn calc_req_fuel(mass: i32) -> i32 {
    let fuel: i32 = mass / 3;
    let missed: i32 = fuel - 2;
    return missed;
}

fn calc_real_req_fuel(mass: i32) -> i32 {
    let mut fuel: i32 = calc_req_fuel(mass);
    let mut total: i32 = 0;
    while fuel > 0 {
        total += fuel;
        fuel = calc_req_fuel(fuel);
    }
    return total;
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename: String = String::from("input.txt");
    let contents: String = fs::read_to_string(filename)?;
    let lines: Lines = contents.lines();

    let numbers = lines.map(|s| s.parse().unwrap());

    let result1: i32 = numbers
        .clone()
        .map(calc_req_fuel)
        .fold(0, |acc: i32, curr: i32| acc + curr);

    let result2: i32 = numbers
        .map(calc_real_req_fuel)
        .fold(0, |acc: i32, curr: i32| acc + curr); //  I could use sum here instead of fold
    println!("Part 1 :\n{:?}", result1);
    println!("Part 2 :\n{:?}", result2);
    Ok(())
}

#[test]
fn test_calc_req_fuel() {
    assert_eq!(calc_req_fuel(12), 2);
    assert_eq!(calc_req_fuel(14), 2);
    assert_eq!(calc_req_fuel(1969), 654);
    assert_eq!(calc_req_fuel(100756), 33583);
}

#[test]
fn test_calc_real_req_fuel() {
    assert_eq!(calc_real_req_fuel(14), 2);
    assert_eq!(calc_real_req_fuel(1969), 966);
    assert_eq!(calc_real_req_fuel(100756), 50346);
}
