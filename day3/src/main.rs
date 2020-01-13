use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Route {
    x: i16,
    y: i16,
    dist: i16,
}

#[warn(dead_code)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct CableWay {
    double: bool,
    cross_type: i8,
    x: i16,
    y: i16,
}

// impl Position {
// }

fn find_wire_distance(wire_1: &str, wire_2: &str) -> i16 {
    let _red_wire_commands: Vec<Route> = str_to_wire(wire_1);
    let _green_wire_commands: Vec<Route> = str_to_wire(wire_2);

    let mut _route_grid: &HashMap<String, CableWay> = &HashMap::new();

    let _start_pos = Position { x: 0, y: 0 };
    let mut last_pos = Position { x: 0, y: 0 };
    let index: HashMap<String, CableWay> = _red_wire_commands.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<String, CableWay>, _route: &Route| {
            for _x in 0.._route.dist {
                last_pos.x = last_pos.x + _route.x;
                last_pos.y = last_pos.y + _route.y;
                let key: &str = &format!("{}_{}", last_pos.x, last_pos.y);
                if acc.contains_key(key) {
                    let cable: &CableWay = acc.get(key).unwrap();
                    if cable.cross_type != 1 {
                        acc.insert(
                            key.to_string(),
                            CableWay {
                                double: true,
                                x: last_pos.x,
                                y: last_pos.y,
                                cross_type: 1,
                            },
                        );
                    }
                } else {
                    acc.insert(
                        key.to_string(),
                        CableWay {
                            double: false,
                            x: last_pos.x,
                            y: last_pos.y,
                            cross_type: 1,
                        },
                    );
                }
            }
            return acc;
        },
    );

    last_pos = Position { x: 0, y: 0 };
    let index2: HashMap<String, CableWay> = _green_wire_commands.iter().fold(
        index,
        |mut acc: HashMap<String, CableWay>, _route: &Route| {
            for _x in 0.._route.dist {
                last_pos.x = last_pos.x + _route.x;
                last_pos.y = last_pos.y + _route.y;
                let key: &str = &format!("{}_{}", last_pos.x, last_pos.y);
                if acc.contains_key(key) {
                    let cable: &CableWay = acc.get(key).unwrap();
                    if cable.cross_type != 2 {
                        acc.insert(
                            key.to_string(),
                            CableWay {
                                double: true,
                                x: last_pos.x,
                                y: last_pos.y,
                                cross_type: 1,
                            },
                        );
                    }
                } else {
                    acc.insert(
                        key.to_string(),
                        CableWay {
                            double: false,
                            x: last_pos.x,
                            y: last_pos.y,
                            cross_type: 2,
                        },
                    );
                }
            }
            return acc;
        },
    );
    // println!("{:#?}", index2);

    let crosses: Vec<&CableWay> = index2
        .iter()
        .filter(|(_key, x)| x.double)
        .map(|(_key, value)| value)
        .collect();

    let mut total: Vec<i16> = crosses
        .iter()
        .map(|cable_way| i16::abs(cable_way.x) + i16::abs(cable_way.y))
        .collect();

    total.sort();

    println!("{:?}", total);
    return total[0];
    // println!("{}", crosses.len());
    // println!("{:?}", index2.iter().filter(|(key, x)| x.double));
    // println!("{:?}", index2);
}

fn str_to_wire(str_wire: &str) -> Vec<Route> {
    str_wire
        .split(",")
        .map(|x| {
            let command = x.chars().next().unwrap();
            let dist: i16 = x[1..x.len()]
                .to_string()
                .parse()
                .expect("Could not parse to i16");
            return match command {
                'L' => Route {
                    x: -1,
                    y: 0,
                    dist: dist,
                },
                'R' => Route {
                    x: 1,
                    y: 0,
                    dist: dist,
                },
                'U' => Route {
                    x: 0,
                    y: -1,
                    dist: dist,
                },
                'D' => Route {
                    x: 0,
                    y: 1,
                    dist: dist,
                },
                d => panic!("Command not found -> {}", d),
            };
        })
        .collect()
}

fn main() {
    let filename: &str = "input.txt";
    let _contents: String = fs::read_to_string(filename).expect("Could not read file");

    let _s: &str = "demneme";

    let mut _route_grid: &HashMap<&str, CableWay> = &HashMap::new();
    let mut lines = _contents.lines();
    let first_wire = lines.next().expect("First wire could not be found!");
    let second_wire = lines.next().expect("Second wire could not be found!");

    let result = find_wire_distance(first_wire, second_wire);

    println!("RESULT {} ", result);
}

#[test]
fn test_opcode_interpreter() {
    let f2 = "R8,U5,L5,D3";
    let f1 = "U7,R6,D4,L4";
    assert_eq!(find_wire_distance(f2, f1), 6);
}

#[test]
fn test_opcode_interpreter2() {
    let f3 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    let f4 = "U62,R66,U55,R34,D71,R55,D58,R83";
    assert_eq!(find_wire_distance(f3, f4), 159);
}

#[test]
fn test_opcode_interpreter3() {
    let f3 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    let f4 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    assert_eq!(find_wire_distance(f3, f4), 135);
}
