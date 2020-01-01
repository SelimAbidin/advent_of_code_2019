use std::fs;

fn main() {
    let filename: &str = "input.txt";
    let contents: String = fs::read_to_string(filename).expect("Could not read file");
    let mut s: Vec<&str> = contents.split(',').collect();
    s[1] = "12";
    s[2] = "2";
    let st: String = s.join(",");
    let full_result = opcode(st);
    println!("Full Result {:?}", full_result);
    println!(
        "Full Result {:?}",
        full_result.split(',').nth(0).unwrap().to_string()
    );
}

fn opcode(code: String) -> String {
    let mut codes: Vec<i32> = code
        .split(",")
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    let mut cursor: usize = 0;
    while codes[cursor] != 99 {
        let operator: i32 = codes[cursor];
        cursor = cursor + 1;
        let v1: i32 = codes[codes[cursor] as usize];
        cursor = cursor + 1;
        let v2: i32 = codes[codes[cursor] as usize];
        cursor = cursor + 1;
        let result_index: usize = codes[cursor] as usize;
        cursor = cursor + 1;
        if operator == 1 {
            let result: i32 = v1 + v2;
            codes[result_index] = result;
        } else if operator == 2 {
            let result: i32 = v1 * v2;
            codes[result_index] = result;
        }
    }

    return codes
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

#[test]
fn test_opcode_interpreter() {
    assert_eq!(
        opcode(String::from("1,0,0,0,99")),
        String::from("2,0,0,0,99")
    );
    assert_eq!(
        opcode(String::from("2,3,0,3,99")),
        String::from("2,3,0,6,99")
    );
    assert_eq!(
        opcode(String::from("2,4,4,5,99,0")),
        String::from("2,4,4,5,99,9801")
    );

    assert_eq!(
        opcode(String::from("1,1,1,4,99,5,6,0,99")),
        String::from("30,1,1,4,2,5,6,0,99")
    );
}
