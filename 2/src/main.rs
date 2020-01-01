use std::fs;

fn main() {
    let filename: &str = "input.txt";
    let contents: String = fs::read_to_string(filename).expect("Could not read file");

    let mut noun_counter = 0;
    let mut nound_done = false;
    while !nound_done {
        let mut verb_done = false;
        let mut verb_counter = 0;
        while !verb_done {
            let mut s: Vec<&str> = contents.split(',').collect();
            let next_noun = &(noun_counter).to_string();
            let next_verb = &(verb_counter).to_string();
            s[1] = next_noun;
            s[2] = next_verb;
            let st: String = s.join(",");
            let full_result = opcode(st);
            //println!("Full Result {:?}", full_result);

            let instruction: String = full_result.split(',').nth(0).unwrap().to_string();
            // println!("Full Result {:?}", instruction);

            if instruction == "19690720" {
                println!("Answer : {} ", 100 * noun_counter + verb_counter);
                println!("Verb : {} Verb : {}", next_noun, next_verb);
                println!("Bulundu {:?}", instruction);
                verb_done = true;
                nound_done = true;
            }
            verb_counter = verb_counter + 1;
            if verb_counter > 90 {
                verb_done = true;
            }
        }
        if noun_counter > 99 {
            nound_done = true;
        }
        noun_counter = noun_counter + 1;
    }
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
            // println!("====> {}", result);
            codes[result_index] = result;
        } else if operator == 2 {
            let result: i32 = v1 * v2;
            //println!("====> {}", result);
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
